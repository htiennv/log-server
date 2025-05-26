# Log Server

A simple HTTP log server written in Rust using Axum.

## Docker Usage

### Building the Docker Image

```bash
docker build -t log-server .
```

### Running with Docker

```bash
# Create a logs directory for persistent storage
mkdir -p logs

# Run the container
docker run -d \
  --name log-server \
  -p 8080:8080 \
  -v $(pwd)/logs:/app/logs \
  -e LOG_PATH=/app/logs/server.log \
  log-server
```

### Running with Docker Compose

```bash
# Start the service
docker-compose up -d

# View logs
docker-compose logs -f

# Stop the service
docker-compose down
```

## API Usage

### Send a log entry

```bash
curl -X POST http://localhost:8080/log \
  -H "Content-Type: application/json" \
  -d '{"data": "This is a test log message"}'
```

## Environment Variables

- `LOG_PATH`: Path to the log file (default: `server.log`)
- `RUST_LOG`: Log level for the application (default: `info`)

## Volumes

- `/app/logs`: Directory where log files are stored
