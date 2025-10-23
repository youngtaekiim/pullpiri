FROM node:22-alpine

RUN apk add --no-cache nginx

# Working directory
WORKDIR /app

# Copy only the package meta first to utilize the cache.
COPY package.json package-lock.json* ./
RUN npm install --no-audit --no-fund \
 && mkdir -p /tmp/nginx/logs /tmp/nginx/tmp /tmp/nginx/run \
 && cp -r node_modules /tmp/node_modules \
 && rm -rf node_modules \
 && ln -s /tmp/node_modules node_modules

# Copy the remaining source (node_modules will be excluded with .dockerignore)
COPY . .

# Copy nginx configuration (using files in source)
COPY nginx.conf /etc/nginx/nginx.conf
COPY proxy.conf /etc/nginx/conf.d/proxy.conf

# Copy entrypoint and execute permissions
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh \
    && chmod -R 777 /app/node_modules

ENV DASHBOARD_ENV=local
EXPOSE 5173 8082
ENTRYPOINT ["/entrypoint.sh"]
