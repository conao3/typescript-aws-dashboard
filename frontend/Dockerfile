FROM node:22-slim AS builder

RUN corepack enable pnpm

WORKDIR /app

COPY package.json pnpm-lock.yaml* ./
RUN pnpm install --frozen-lockfile

COPY . .

RUN pnpm build

FROM node:22-slim

RUN corepack enable pnpm

WORKDIR /app

COPY --from=builder /app/.output /app/.output
COPY --from=builder /app/package.json /app/package.json

ENV NODE_ENV=production
ENV PORT=17232

EXPOSE 17232

CMD ["node", ".output/server/index.mjs"]
