FROM ubuntu:20.04

WORKDIR /app

RUN apt-get update && apt-get install -y python3-pyqt5
COPY ./main.py ./main.py

ARG VERSION
ENV VERSION ${VERSION}

CMD python3 main.py

