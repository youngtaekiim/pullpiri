FROM python:3.9.19-slim

WORKDIR /app

RUN apt update && apt install -y python3-pyqt5
RUN pip install cyclonedds PyQt5
COPY . .

CMD python3 /app/main.py

