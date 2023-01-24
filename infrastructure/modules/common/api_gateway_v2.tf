resource "aws_apigatewayv2_api" "harbor_api_gateway" {
  name                       = "example-websocket-api"
  protocol_type              = "WEBSOCKET"
  route_selection_expression = "$request.body.action"
}
