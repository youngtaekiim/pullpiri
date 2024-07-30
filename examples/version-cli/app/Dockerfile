FROM python:3.9.19-slim

WORKDIR /app

COPY ./main.py ./main.py

ARG VERSION
ENV VERSION ${VERSION}

CMD python3 main.py

