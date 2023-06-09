require "aws-sdk-dynamodb"

DB = ::Aws::DynamoDB::Client.new
LANG = "ruby-yjit-3.2-x86"

def handler(event:, context:)
  return { statusCode: 400, body: "yjit not enabled"} unless RubyVM::YJIT.enabled? 
  
  event_params = parse_event_body_params(event)
  iteration = event_params["iteration"]

  return { statusCode: 400 } unless iteration

  new_item = DB.put_item(
    table_name: ENV["TABLE"],
    item: { **event_params,
            "langCase" => LANG,
            "iteration" => iteration,
            "raw_event" => event.to_h
    },
    return_values: "ALL_OLD").attributes

  previous_item = DB.get_item(table_name: ENV["TABLE"], key: { langCase: LANG, iteration: iteration - 1 }).item

  { statusCode: 200, body: (previous_item || new_item).to_json }
end

def parse_event_body_params(event)
  JSON.parse(event["isBase64Encoded"] ? Base64.decode64(event["body"]) : event["body"])
end
