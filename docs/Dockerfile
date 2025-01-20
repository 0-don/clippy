# Install dependencies only when needed
# Stage 0
FROM imbios/bun-node AS deps
WORKDIR /app

COPY package.json ./
RUN bun install
#############################################


# Rebuild the source code only when needed
# Stage 1
FROM imbios/bun-node AS builder
WORKDIR /app

COPY . .
COPY --from=deps /app/node_modules ./node_modules


RUN bun run build
#############################################


# Production image, copy only production files
# Stage 2
FROM imbios/bun-node AS prod

WORKDIR /app

COPY --from=builder /app/public ./public
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/node_modules ./node_modules

ENV HOST=0.0.0.0
ENV PORT=4321
EXPOSE 4321
CMD bun ./dist/server/entry.mjs
#############################################
