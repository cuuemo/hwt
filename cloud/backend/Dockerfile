# syntax=docker/dockerfile:1

FROM python:3.12-slim
WORKDIR /app

COPY backend/requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY backend/ .

# Vue SPA — built ahead of time via `cd cloud/frontend && npm run build`.
# Operators must rebuild `frontend/dist/` before `docker compose up --build`
# whenever frontend sources change.
COPY frontend/dist /app/frontend_dist

# RSA 密钥目录（挂载持久化）
RUN mkdir -p keys

EXPOSE 10000

CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "10000", "--workers", "2"]
