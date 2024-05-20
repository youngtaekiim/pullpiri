import sys

from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import QApplication, QWidget, QHBoxLayout, QVBoxLayout, QRadioButton, QLabel

class MainWindow(QWidget):
    def __init__(self):
        super().__init__()
        self.initUI()

    def initUI(self):
        layout = QHBoxLayout()
        self.label = QLabel()
        self.label.setFixedSize(400,200)
        self.label.setAlignment(Qt.AlignCenter)
        ver = os.environ.get('VERSION')
        self.label.setText(f"Version: {ver}")

        layout.addWidget(self.label)

        self.setLayout(layout)
        self.setWindowTitle("version")
        self.show()

if __name__ == '__main__':
    app = QApplication(sys.argv)
    ex = MainWindow()
    sys.exit(app.exec_())
