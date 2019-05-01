FROM alpine:latest
RUN apk update --no-cache
RUN apk add --no-cache curl jq

ENV TRIGGER_VERSION 1.1.1
ENV TRIGGER_PORT 4567

EXPOSE $TRIGGER_POSTS

ADD docker-entrypoint.sh /docker-entrypoint.sh

RUN curl -L "https://github.com/RedL0tus/trigger/releases/download/$TRIGGER_VERSION/trigger" -o /usr/bin/trigger
RUN chmod u+x /usr/bin/trigger

WORKDIR /work

ENTRYPOINT ["/bin/sh", "/docker-entrypoint.sh"]
CMD ["trigger"]
