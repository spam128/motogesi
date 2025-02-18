# =============================
# 1. Stage: Builder (install dependencies)
# =============================
FROM python:3.12-slim AS builder

WORKDIR /app

# system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    gcc \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# install dependencies
COPY requirements.txt .
RUN pip install --upgrade pip && \
    pip install --no-cache-dir --prefix=/install -r requirements.txt

# =============================
# 2. Stage: prod image
# =============================
FROM python:3.12-slim AS runtime

# create non-root user
RUN addgroup --system app && adduser --system --group app
USER app

WORKDIR /app
COPY --from=builder /install /usr/local
COPY . .

CMD ["gunicorn", "--bind", "0.0.0.0:8000", "core.wsgi:application"]
