FROM node:lts-bookworm-slim@sha256:18aacc7993a16f1d766c21e3bff922e830bcdc7b549bbb789ceb7374a6138480 as builder
WORKDIR /docs

# To solve "FATAL ERROR: Reached heap limit Allocation failed - JavaScript heap out of memory"
ENV NODE_OPTIONS="--max_old_space_size=4096"

COPY ../openapi .

RUN npm update -g npm
RUN npm install redoc-cli -g

RUN redoc-cli build openapi.yaml --options.theme.colors.primary.main=blue

FROM nginx:alpine@sha256:2d2a2257c6e9d2e5b50d4fbeb436d8d2b55631c2a89935a425b417eb95212686 as runtime
LABEL maintainer="Carlo Micieli <mail@trenako.com>"
LABEL description="The trenako openapi documentation"

COPY --from=builder /docs/redoc-static.html /usr/share/nginx/html/index.html
