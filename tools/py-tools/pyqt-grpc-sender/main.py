# SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
# SPDX-License-Identifier: Apache-2.0

import sys
import grpc
import yamlparser_pb2_grpc as pb2_grpc
import yamlparser_pb2 as pb2

from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import QApplication, QWidget, QHBoxLayout, QVBoxLayout, QRadioButton, QLabel

class SendClient(object):
    def __init__(self):
        self.host = os.environ.get('HOST_IP')
        self.server_port = 47004

        self.channel = grpc.insecure_channel(
            '{}:{}'.format(self.host, self.server_port))

        self.stub = pb2_grpc.ConnectionStub(self.channel)

    def get_url(self, message):
        req = pb2.SendRequest(request=message)
        print(f'{message}')
        return self.stub.Send(req)

class MainWindow(QWidget):
    def __init__(self):
        super().__init__()

        self.initUI()

    def initUI(self):
        layout = QHBoxLayout()
        
        buttonWidget = QWidget()
        buttonLayout = QVBoxLayout(buttonWidget)
        buttonWidget.setStyleSheet("background-color: lightgrey; border:1px solid black;")
        self.buttons = []
        for label in ['update (v2.0)', 'rollback (v1.0)']:
            btn = QRadioButton(label)
            btn.setCheckable(True)
            btn.clicked.connect(self.onButtonClicked)
            btn.setFixedSize(200, 50)
            btn.setStyleSheet("border:1px solid black;")
            buttonLayout.addWidget(btn)
            self.buttons.append(btn)

        self.label = QLabel()
        self.label.setFixedSize(100,200)
        self.label.setAlignment(Qt.AlignCenter)
        
        layout.addWidget(buttonWidget)
        layout.addWidget(self.label)

        self.setLayout(layout)
        self.setWindowTitle('Qt gRPC sender')
        self.show()

    def onButtonClicked(self):
        clicked_button = self.sender()
        for btn in self.buttons:
            if btn is clicked_button:
                btn.setStyleSheet("background-color: green")
                self.label.setText(btn.text()) #클릭된 버튼표시
                client = SendClient()
                if "update" in btn.text():
                    result = client.get_url(message="example/update-scenario.yaml")
                elif "rollback" in btn.text():
                    result = client.get_url(message="example/rollback-scenario.yaml")
                
            else:
                btn .setStyleSheet("") #색상 되돌림
if __name__ == '__main__':
    app = QApplication(sys.argv)
    ex = MainWindow()
    sys.exit(app.exec_())
