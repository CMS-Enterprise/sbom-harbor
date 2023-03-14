```mermaid
sequenceDiagram
    participant Batch Task
    participant Pushgateway
    participant Prometheus
    Batch Task ->> Pushgateway: Push metric
    loop
        Pushgateway->>Pushgateway: Wait for scrape
    end
    Prometheus->>Pushgateway: Scrape on interval
     loop
        Pushgateway->>Pushgateway: Flush
    end   
```