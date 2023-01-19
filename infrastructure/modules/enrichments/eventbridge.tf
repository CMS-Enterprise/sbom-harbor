resource "aws_cloudwatch_event_bus" "enrichments" {
  name = "${var.environment}-harbor-enrichments"
}

output "enrichments_event_bus_id" {
  value = aws_cloudwatch_event_bus.enrichments.id
}

output "enrichments_event_bus_arn" {
  value = aws_cloudwatch_event_bus.enrichments.arn
}
