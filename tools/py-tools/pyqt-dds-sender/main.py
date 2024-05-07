import sys
from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import QApplication, QWidget, QHBoxLayout, QVBoxLayout, QRadioButton, QLabel
from cyclonedds.domain import DomainParticipant
from cyclonedds.pub import DataWriter
from cyclonedds.topic import Topic
from cyclonedds.util import duration

import gearState
from gearState import DataType

class MainWindow(QWidget):
    def __init__(self):
        super().__init__()

        # DDS 설정
        self.participant = DomainParticipant()
        self.topic = Topic(self.participant, "rt/piccolo/gear_state", DataType)
        self.writer = DataWriter(self.participant, self.topic)

        self.initUI()

    def initUI(self):
        layout = QHBoxLayout()
        
        buttonWidget = QWidget()
        buttonLayout = QVBoxLayout(buttonWidget)
        buttonWidget.setStyleSheet("background-color: lightgrey; border:1px solid black;")  # 버튼 레이아웃 스타일 설정
        self.buttons = []
        for label in ['drive', 'parking', 'neutral', 'reverse']:
            btn = QRadioButton(label)
            btn.setCheckable(True)
            btn.clicked.connect(self.onButtonClicked)
            btn.setFixedSize(100, 50)
            btn.setStyleSheet("border:1px solid black;")  # 버튼 스타일 설정
            buttonLayout.addWidget(btn)
            self.buttons.append(btn)

        self.label = QLabel()
        self.label.setFixedSize(100,200)
        self.label.setAlignment(Qt.AlignCenter)
        
        layout.addWidget(buttonWidget)
        layout.addWidget(self.label)

        self.setLayout(layout)
        self.setWindowTitle('Qt Button Example with DDS')
        self.show()

    def onButtonClicked(self):
        clicked_button = self.sender()
        for btn in self.buttons:
            if btn is clicked_button:
                btn.setStyleSheet("background-color: green")
                self.label.setText(btn.text()) #클릭된 버튼표시
                # DDS 메시지 전송
                
                gear = clicked_button.text()
                data = gearState.DataType(gear)
                self.writer.write(data)
            else:
                btn .setStyleSheet("") #색상 되돌림
if __name__ == '__main__':
    app = QApplication(sys.argv)
    ex = MainWindow()
    sys.exit(app.exec_())
