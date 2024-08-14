import sys
from PyQt5.QtWidgets import *
from PyQt5.QtGui import *
from PyQt5.QtCore import *

from cyclonedds.domain import DomainParticipant
from cyclonedds.pub import DataWriter
from cyclonedds.topic import Topic
from cyclonedds.util import duration

import GearState
import DayTime
import Speed
import TurnLight
import LightState

class MainWindow(QWidget):
    def __init__(self):
        super().__init__()

        # DDS 설정
        self.participant1 = DomainParticipant()
        self.participant2 = DomainParticipant()
        self.participant3 = DomainParticipant()
        self.participant4 = DomainParticipant()

        self.topic_gear = Topic(self.participant1, "/rt/piccolo/Gear_State", GearState.DataType)
        self.topic_dayTime = Topic(self.participant2, "/rt/piccolo/Day_Time", DayTime.DataType)
        #self.topic_speed = Topic(self.participant3, "/rt/piccolo/Speed", Speed.DataType)
        self.topic_speed = Topic(self.participant3, "/rt/piccolo/Light_State", LightState.DataType)
        self.topic_turnLight = Topic(self.participant4, "/rt/piccolo/Turn_Light", TurnLight.DataType)

        self.writer_gear = DataWriter(self.participant1, self.topic_gear)
        self.writer_dayTime = DataWriter(self.participant2, self.topic_dayTime)
        self.writer_speed = DataWriter(self.participant3, self.topic_speed)
        self.writer_turnLight = DataWriter(self.participant4, self.topic_turnLight)

        self.initUI()

    def initUI(self):
        layout = QHBoxLayout()

        L_layout = QVBoxLayout()
        R_layout = QVBoxLayout()

        self.dayTimeMonitor = QGroupBox("day")
        self.dayTimeMonitor.setStyleSheet("background-color: gold;"
                                     "color : slateblue;"
                                     "font-size : 16px;")

        self.ddaMonitor = QTextEdit("")

        L_layout.addWidget(self.dayTimeMonitor)
        L_layout.addWidget(self.ddaMonitor)
        L_layout.setStretchFactor(self.dayTimeMonitor, 3)
        L_layout.setStretchFactor(self.ddaMonitor, 1)

        lightLayout = QHBoxLayout()
        lightLayout.setAlignment(Qt.AlignCenter)
        self.dayTimeMonitor.setLayout(lightLayout)

        self.L_light = QLabel()
        self.R_light = QLabel()
        lightLayout.addWidget(self.L_light)
        lightLayout.addWidget(self.R_light)
        self.light_w = int(self.L_light.geometry().width()/4)
        self.light_h = int(self.L_light.geometry().height()/4)
        self.ddaMonitor.setText("self.light_w = %s self.light_h = %s" %(self.light_w, self.light_h))

        light = QPixmap(self.light_w,self.light_h)
        light.fill(QColor("#00ff0000"))
        self.L_light.setPixmap(light)
        self.R_light.setPixmap(light)

        daytimeButtonWidget = QWidget()
        daytimeButtonLayout = QHBoxLayout(daytimeButtonWidget)
        daytimeButtonWidget.setStyleSheet("background-color: lightgrey; border:1px solid black;")  # 버튼 레이아웃 스타일 설정
        self.dayTimeButtons = []
        for label in ['day', 'night']:
            btn = QRadioButton(label)
            btn.setCheckable(True)
            btn.clicked.connect(self.onDayTimeButtonClicked)
            btn.setFixedSize(100, 50)
            btn.setStyleSheet("border:1px solid black;")  # 버튼 스타일 설정
            daytimeButtonLayout.addWidget(btn)
            self.dayTimeButtons.append(btn)

        lightButtonWidget = QWidget()
        lightButtonLayout = QHBoxLayout(lightButtonWidget)
        lightButtonWidget.setStyleSheet("background-color: lightgrey; border:1px solid black;")  # 버튼 레이아웃 스타일 설정
        self.lightButtons = []
        for label in ['TurnOn', 'TurnOff']:
            btn = QRadioButton(label)
            btn.setCheckable(True)
            btn.clicked.connect(self.onLightButtonClicked)
            btn.setFixedSize(100, 50)
            btn.setStyleSheet("border:1px solid black;")  # 버튼 스타일 설정
            lightButtonLayout.addWidget(btn)
            self.lightButtons.append(btn)

        gearButtonWidget = QWidget()
        gearButtonLayout = QHBoxLayout(gearButtonWidget)
        gearButtonWidget.setStyleSheet("background-color: lightgrey; border:1px solid black;")  # 버튼 레이아웃 스타일 설정
        self.gearButtons = []
        for label in ['drive', 'parking', 'nuetral', 'reserve']:
            btn = QRadioButton(label)
            btn.setCheckable(True)
            btn.clicked.connect(self.onGearButtonClicked)
            btn.setFixedSize(100, 50)
            btn.setStyleSheet("border:1px solid black;")  # 버튼 스타일 설정
            gearButtonLayout.addWidget(btn)
            self.gearButtons.append(btn)

        
        speedButtonWidget = QWidget()
        speedButtonLayout = QHBoxLayout(speedButtonWidget)
        speedButtonWidget.setStyleSheet("background-color: lightgrey; border:1px solid black;")  # 버튼 레이아웃 스타일 설정
        self.speedMonitor = QLabel()
        self.speedMonitor.setAlignment(Qt.AlignCenter)
        speedButtonLayout.addWidget(self.speedMonitor)
        self.speedMonitor.setText("0")
        self.speedButtons = []
        for label in ['SpeedUp', 'SpeedDown']:
            btn = QPushButton(label)
            btn.setCheckable(True)
            btn.clicked.connect(self.onSpeedButtonClicked)
            btn.setFixedSize(100, 50)
            btn.setStyleSheet("border:1px solid black;")  # 버튼 스타일 설정
            speedButtonLayout.addWidget(btn)
            self.speedButtons.append(btn)


#        self.label = QLabel()
#        self.label.setFixedSize(100,200)
#        self.label.setAlignment(Qt.AlignCenter)

        R_layout.addWidget(daytimeButtonWidget)        
        R_layout.addWidget(lightButtonWidget)        
        R_layout.addWidget(gearButtonWidget)
        R_layout.addWidget(speedButtonWidget)
#        R_layout.addWidget(self.label)

        layout.addLayout(L_layout)
        layout.addLayout(R_layout)

        self.setLayout(layout)
        self.setWindowTitle('Dummy Light Reconcile')
        self.show()

    def onDayTimeButtonClicked(self):
        clicked_button = self.sender()
        for btn in self.dayTimeButtons:
            if btn is clicked_button:
                btn.setStyleSheet("background-color: green")
#                self.label.setText(btn.text()) #클릭된 버튼표시
                # DDS 메시지 전송
                
                day = False
                if clicked_button.text() == 'day':
                    day = True
                    self.dayTimeMonitor.setTitle("day")
                    self.dayTimeMonitor.setStyleSheet("background-color: gold;"
                                     "color : slateblue;"
                                     "font-size : 16px;")
                    self.ddaMonitor.setText("send dds day")
                elif clicked_button.text() == 'night':
                    day = False
                    self.dayTimeMonitor.setTitle("night")
                    self.dayTimeMonitor.setStyleSheet("background-color: darkgrey;"
                                     "color : yellow;"
                                     "font-size : 16px;")
                    self.ddaMonitor.setText("send dds night")

                data = DayTime.DataType(day)
                self.writer_dayTime.write(data)
            else:
                btn .setStyleSheet("") #색상 되돌림

    def onLightButtonClicked(self):
        clicked_button = self.sender()
        for btn in self.lightButtons:
            if btn is clicked_button:
                btn.setStyleSheet("background-color: green")
#                self.label.setText(btn.text()) #클릭된 버튼표시
                # DDS 메시지 전송
                
                on = "TurnOff"
                if clicked_button.text() == 'TurnOn':
                    on = "on"
                    
                    light = QPixmap(self.light_w,self.light_h)
                    light.fill(QColor("floralwhite"))
                    self.L_light.setPixmap(light)
                    self.R_light.setPixmap(light)
                    self.ddaMonitor.setText("send dds TurnOn")
                elif clicked_button.text() == 'TurnOff':
                    on = "off"
                    light = QPixmap(self.light_w,self.light_h)
                    light.fill(QColor("#00ff0000"))
                    self.L_light.setPixmap(light)
                    self.R_light.setPixmap(light)
                    self.ddaMonitor.setText("send dds TurnOff")

                data = TurnLight.DataType(on)
                self.writer_turnLight.write(data)
            else:
                btn .setStyleSheet("") #색상 되돌림

    def onGearButtonClicked(self):
        clicked_button = self.sender()
        for btn in self.gearButtons:
            if btn is clicked_button:
                btn.setStyleSheet("background-color: green")
#                self.label.setText(btn.text()) #클릭된 버튼표시
                # DDS 메시지 전송
                
                gear = clicked_button.text()
                data = GearState.DataType(gear)
                self.writer_gear.write(data)
                self.ddaMonitor.setText("send dds "+gear)
            else:
                btn .setStyleSheet("") #색상 되돌림

    def onSpeedButtonClicked(self):
        clicked_button = self.sender()
        for btn in self.speedButtons:
            if btn is clicked_button:
                btn.setStyleSheet("")
#                self.label.setText(btn.text()) #클릭된 버튼표시
                # DDS 메시지 전송

                #value = int(self.speedMonitor.text())
                value = False
                if clicked_button.text() == 'SpeedUp':
                    #value += 1
                    value = True
                elif clicked_button.text() == 'SpeedDown':
                    #if value > 0:
                    #    value -= 1
                    value = False

                #data = Speed.DataType(value)
                data = LightState.DataType(value)
                self.writer_speed.write(data)
                self.ddaMonitor.setText("send dds speed")
                self.speedMonitor.setText(str(value))
            else:
                btn .setStyleSheet("") #색상 되돌림

if __name__ == '__main__':
    app = QApplication(sys.argv)
    ex = MainWindow()
    sys.exit(app.exec_())
