FROM alpine:latest

WORKDIR /app

COPY ./build/ws-and-api /app/ws-and-api

CMD ["./ws-and-api"]