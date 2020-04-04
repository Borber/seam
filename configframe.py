from PyQt5.QtWidgets import *
from PyQt5.QtCore import *
from PyQt5.QtGui import *
import json
import sys
from os import path,_exit
import traceback

class cstmBtn(QPushButton):
    def __init__(self,string,parent):
        super().__init__(string,parent)
        self.initUI()
    def initUI(self):
        # self.setWindowFlags(Qt.FramelessWindowHint)
        # self.setText('''<div>'''+self.caption+r'''</div>''')
        self.resize(35,25)
    def setStyle(self,normal,hover,pressed,fontsize):
        self.setStyleSheet("QPushButton{{border-style:none;color:white;font-size:{fontsize};background:{normal};}}QPushButton:hover{{background:{hover};}}QPushButton:pressed{{background:{pressed};}}".format(normal=normal,hover=hover,pressed=pressed,fontsize=fontsize))

class MainFrame(QWidget):
    '''啦啦啦啦啦
    '''
    def __init__(self):
        super().__init__()
        self.initUI()
    def initUI(self):
        # 窗口大小位置标题
        self.resize(640,320)
        self.move ((app.desktop().width() - self.width())/2,(app.desktop().height() - self.height())/2)
        self.setWindowTitle('SBtream Config')
        self.setWindowFlags(Qt.FramelessWindowHint)
        
        # 标题栏
        self.topbar=QLabel(self) 
        self.topbar.resize(640,25)
        self.topbar.setStyleSheet("QLabel{color:white;font-size:32px;background:#222222;}")

        # 设置面板
        self.panel=QLabel(self) 
        self.panel.resize(640,320-25)
        self.panel.move(0,25)
        self.panel.setStyleSheet("QLabel{border:1px solid #222222;}")

        # 关闭按钮
        self.closebtn=cstmBtn("×",self)
        self.closebtn.setStyle("#222222","#ff0000","#e10000","18px")
        self.closebtn.move(605,0)

        # 最小化按钮
        self.minimize=cstmBtn("—",self)
        self.minimize.setStyle("#222222","#666666","#666666","14px")
        self.minimize.move(570,0)

        # 标题
        self.title=QLabel(self.topbar)
        self.title.setStyleSheet("QLabel{font-size:12px;background:transparent;color:white;}")
        self.title.setText("SBtream Config")
        self.title.move(15,-18)
        self.title.setMinimumHeight(60)

        # 绑定事件
        self.closebtn.clicked.connect(self.close)
        self.minimize.clicked.connect(self.showMinimized)
        
        
        # 载入json
        with open("config/config.json", "r", encoding='utf-8') as load_f:
            load_dict = json.load(load_f)
        self.set_num=len(load_dict["index"])

        # 初始化流体布局
        self.args=[]
        self.innerpanel=QWidget(self.panel)

        # 滚动条
        sca=QScrollArea(self.panel)
        sca.resize(640-2,320-25-2)
        sca.move(1,1)
        sca.setStyleSheet("QScrollArea{border:none;}")
        sca.setWidget(self.innerpanel)

        #布局
        innercontents=QVBoxLayout()
        innercontents.setContentsMargins(20,20,20,20)
        self.grid=QGridLayout()
        buttoncontents=QHBoxLayout()
        bilicontents=QHBoxLayout()
        bili=QWidget(self.innerpanel)
        table=QWidget(self.innerpanel)
        table.setLayout(self.grid)
        bili.setLayout(bilicontents)
        button=QWidget(self.innerpanel)
        button.setLayout(buttoncontents)
        innercontents.addWidget(bili)
        innercontents.addWidget(table)
        # innercontents.addItem(QSpacerItem(0,280,QSizePolicy.Minimum,QSizePolicy.Minimum))
        innercontents.addWidget(button)
        self.innerpanel.setLayout(innercontents)
        
        #bili
        blabel=QLabel(bili)
        blabel.setText("Bilibili Cookie")
        blabel.setStyleSheet("QLabel{font-size:12px;border:none;margin-left:10px}")
        self.binputbox=QLineEdit(load_dict["bilibili_cookie"],bili)
        self.binputbox.setFixedHeight(25)
        self.binputbox.setMinimumWidth(300)
        self.binputbox.setTextMargins(0,0,25,0)
        self.binputbox.setStyleSheet("QLineEdit{font-size:12px;}")
        bilicontents.addWidget(blabel)
        bilicontents.addWidget(self.binputbox)
        bilicontents.addItem(QSpacerItem(640,0,QSizePolicy.Minimum,QSizePolicy.Minimum))

        #新建设置项函数
        def addsettings(name,value,position):
            label=QLabel(table)
            label.setText(name)
            label.setStyleSheet("QLabel{font-size:12px;border:none;margin-left:10px}")
            if name=="kind":
                inputbox=QComboBox(table)
                inputbox.addItems(self.all_kind)
                inputbox.setCurrentText(value)
                inputbox.setMinimumWidth(100)
            else:
                inputbox=QLineEdit(value,table)
                inputbox.setTextMargins(0,0,25,0)
            inputbox.setFixedHeight(25)
            inputbox.setStyleSheet("QLineEdit{font-size:12px;}")
            self.args.append(inputbox)
            self.grid.addWidget(inputbox,position[0],2*position[1]+1)
            self.grid.addWidget(label,position[0],2*position[1])
            self.save_num+=1

        #grid 设置项
        self.grid.setVerticalSpacing(10)
        self.grid.setHorizontalSpacing(10)
        self.save_num=0
        self.set_names=["name","kind","rid"] ## 如需新增设置字段修改此处即可
        ## 如果新增了支持平台，需要在下面的列表里加入
        self.all_kind=["bilibili","chushou","douyin","douyu","egame","huajiao","huomao","huya","iqiyi","kuaishou","kugou","longzhu","now","pps","v6cn","wangyicc","xigua","yingke","yizhibo","yy","zhanqi"]
        positons=[(i,j) for i in range(self.set_num) for j in range(len(self.set_names))]
        names=self.set_names*len(load_dict["index"])
        values=[each[i]  for each in load_dict["index"] for i in ["name","kind","rid"]]
        for name,value,position in zip(names,values, positons):
            addsettings(name,value,position)

        

        # 新建按钮
        self.starter=cstmBtn("新增设置项",button)
        self.starter.setStyleSheet("QPushButton{border:none;font-family:wildings;font-size:12px;border-radius: 5px;color:white;background:#888888;}QPushButton:hover{background:#1f96ff}")
        self.starter.setFixedSize(96,32)
        self.starter.clicked.connect(lambda:[addsettings(self.set_names[i],"",[self.set_num,i]) for i in range(len(self.set_names))])
        buttoncontents.addWidget(self.starter)

        # 保存按钮
        self.starter=cstmBtn("保存设置",button)
        self.starter.setStyleSheet("QPushButton{border:none;font-family:wildings;font-size:12px;border-radius: 5px;color:white;background:#888888;}QPushButton:hover{background:#1f96ff}")
        self.starter.setFixedSize(96,32)
        self.starter.clicked.connect(self.save)
        buttoncontents.addWidget(self.starter)

        # 调整内容页大小
        self.innerpanel.resize(620,max(32*(len(load_dict["index"])+1)+36,320-25-2))
        
        # 显示
        self.show()
    
    def save(self):
        save_dict=dict()
        save_dict["bilibili_cookie"]=self.binputbox.text()
        save_dict["index"]=[]
        for i in range(self.save_num//3):
            temp_dict=dict()
            for j in range(3):
                try:temp_dict[self.set_names[j]]=self.args[i*3+j].text()
                except:
                    temp_dict[self.set_names[j]]=self.args[i*3+j].currentText()
            save_dict["index"].append(temp_dict)
        with open("config/config.json", "w", encoding='utf-8') as fp:
            try:json.dump(save_dict,fp)
            except:
                QMessageBox(self).critical(self,"错误","<h1>保存失败</h1>")
                pass
            else:
                QMessageBox(self).information(self,"信息","<h1>保存成功</h1>")
    def closebtnEvent(self,event):
        tip=QMessageBox(self).question(self,"退出？","确认退出？")
        # print(tip.button)
        # print(tip)
        if tip==QMessageBox.Yes:
            event.accept()
        else:
            event.ignore()
        
    def mousePressEvent(self,event):
        '''If you move the widget as a result of the mouse event, use the global position returned by globalPos() to avoid a shaking motion. ——From Qt helper
        '''
        if event.button() == Qt.LeftButton:
            self.mousePressStatus=True
            self.mousePressPosition=event.globalPos()-self.pos()
        event.accept()
    def mouseMoveEvent(self,event):
        if self.mousePressStatus:
            self.move(event.globalPos() - self.mousePressPosition)
        event.accept()
    def mouseReleaseEvent(self,event):
        self.mousePressStatus=False
        event.accept()

if __name__ == "__main__":
    QCoreApplication.setAttribute(Qt.AA_EnableHighDpiScaling)
    app = QApplication(sys.argv)
    try:ex = MainFrame()
    except Exception: 
        traceback.print_exc()
    app.exec_()
