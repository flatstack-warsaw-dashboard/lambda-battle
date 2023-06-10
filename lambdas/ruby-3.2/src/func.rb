require "aws-sdk-dynamodb"
require "json"

DB = ::Aws::DynamoDB::Client.new
LANG_REGEXP = /^\/ruby-(\d)-(\d)-(yjit-)?(x86|arm)$/

Item = Data.define(:langCase, :iteration, :raw_event, :event_params) do
  def initialize(langCase:, iteration:, raw_event: nil, event_params: {}) = super
  def to_h = super.compact.except(:event_params).merge(**event_params)
end

def handler(event:, **)
  event = JSON.parse(event.to_json, symbolize_names: true)
  lang = event[:path].gsub(/\//, '')
  event_params = parse_event_body_params(event) => iteration:

  return { statusCode: 400 } unless iteration
  
  item = Item.new(lang, iteration, event.to_h, event_params) 

  new_item = DB.put_item(
    table_name: ENV["TABLE"], 
    item: item.to_h,
    return_values: "ALL_OLD"
  ).attributes

  previous_item = DB.get_item(
    table_name: ENV["TABLE"], 
    key: Item.new(lang, iteration - 1).to_h
  ).item

  { statusCode: 200, body: (previous_item || new_item).to_json }
end

def parse_event_body_params(event) = case event
  in isBase64Encoded: encoded, body: body, path: LANG_REGEXP
    b = encoded ? Base64.decode64(body) : body

    JSON.parse(b, symbolize_names: true)
  end
