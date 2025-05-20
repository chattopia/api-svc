FROM rust:1.87 AS builder

# Compile service
WORKDIR /service
COPY . .
RUN cargo build --release


WORKDIR /rootfs

# Install service
ENV APP_PATH=/rootfs/bin/chattopia-api
RUN mkdir -p $(dirname ${APP_PATH}) \
 && install -s -m 755 /service/target/release/chattopia-api ${APP_PATH}

# Copy service dynamically linked dependencies
RUN apt-get update && apt-get install -y findutils
RUN ldd ${APP_PATH} | grep -oE '/[^ ]+' \
                    | xargs -I '{}' cp --parents '{}' /rootfs/

# Create /etc/passwd file with service user
RUN useradd -u 1000 -r -s /sbin/nologin usvc \
 && mkdir etc \
 && grep '^usvc:' /etc/passwd > etc/passwd


FROM scratch

# The following labels are set dynamically:
# - org.opencontainers.image.version
# - org.opencontainers.image.revision
# - org.opencontainers.image.created

LABEL org.opencontainers.image.title="chattopia-api" \
      org.opencontainers.image.base.name="rust:1.87" \
      org.opencontainers.image.description="Chattopia API microservice backend image" \
      org.opencontainers.image.url="https://github.com/chattopia/chattopia-api" \
      org.opencontainers.image.source="https://github.com/chattopia/chattopia-api.git" \
      org.opencontainers.image.authors="Mario Salazar <mariosb93+chattopia@gmail.com>" \
      org.opencontainers.image.licenses="Apache License 2.0"

COPY --from=builder /rootfs/ /

USER usvc
EXPOSE 8000
ENTRYPOINT ["/bin/chattopia-api"]
