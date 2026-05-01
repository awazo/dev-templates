FROM debian:trixie-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY ./app/myapp /app/myapp
RUN chmod +x /app/myapp
EXPOSE 8080
CMD ["/app/myapp"]

