require "aws-sdk-dynamodb"

DB = ::Aws::DynamoDB::Client.new
LANG = "ruby-yjit-3.2-x86"
ITEM = Data.define(:langCase, :iteration, :raw_event, :event_params) do
  def to_h = super.except(:event_params).merge(**event_params)
end

def handler(event:, **)
  return { statusCode: 400, body: "yjit not enabled"} unless RubyVM::YJIT.enabled? 

  event_params = parse_event_body_params(event)
  event_params => iteration:

  return { statusCode: 400 } unless iteration
  
  item = ITEM.new(LANG, iteration, event.to_h, event_params) 

  new_item = DB.put_item(
    table_name: ENV["TABLE"], 
    item: item.to_h,
    return_values: "ALL_OLD"
  ).attributes

  previous_item = DB.get_item(table_name: ENV["TABLE"], key: { langCase: LANG, iteration: iteration - 1 }).item

  { statusCode: 200, body: (previous_item || new_item).to_json }
end

def parse_event_body_params(event) = JSON.parse(
  (event in isBase64Encoded: TrueClass, body: body) ? Base64.decode64(body) : body, 
  symbolize_names: true
)
