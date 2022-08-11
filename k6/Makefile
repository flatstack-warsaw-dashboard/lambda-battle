build:
	docker build -f Dockerfile -t lambda-battle-k6:latest .
run: build
	docker run --env GATEWAY_URL --env LANG_CASE lambda-battle-k6:latest run /scenarios/sequential.js
