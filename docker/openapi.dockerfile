FROM node:lts as builder
WORKDIR /docs

# To solve "FATAL ERROR: Reached heap limit Allocation failed - JavaScript heap out of memory"
ENV NODE_OPTIONS="--max_old_space_size=4096"

COPY ../openapi .

RUN npm update -g npm
RUN npm install redoc-cli -g

RUN redoc-cli build api-schema.yaml --options.theme.colors.primary.main=blue

FROM nginx:alpine as runtime
COPY --from=builder /docs/redoc-static.html /usr/share/nginx/html/index.html