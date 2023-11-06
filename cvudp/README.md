# CvMat
| 函数名 | 功能 | 参数 | 返回 |
| --- | --- | --- | --- |
| new_cam | 创建一个摄像头对象 | 无 | CvMat 对象指针 |
| get_frame | 从摄像头获取一帧图像到 Frame 结构体 | CvMat 对象指针 | Frame 结构体，其中包含 data、cols 和 rows |
| del_cam | CvMat 对象使用完后必须用该函数释放内存，否则会内存泄漏！ | CvMat 对象指针 | 无 |
# UdpReader
| 函数名 | 功能 | 参数 | 返回 |
| --- | --- | --- | --- |
| udp_reader | 创建一个 UDP 接收器对象 | 无 | UdpReader 对象指针 |
| udp_frame | 从 UDP Socket 接收一帧图像到 Frame 结构体 | UdpReader 对象指针 | Frame 结构体，其中包含 data、cols 和 rows |
| del_udp | UdpReader 对象使用完后必须用该函数释放内存，否则会内存泄漏！ | UdpReader 对象指针 | 无 |
# Send
| 函数名 | 功能 | 参数 | 返回 |
| --- | --- | --- | --- |
| send_video | 创建一个线程并循环发送摄像头图像数据直至收到结束信号 | 无 | 无 |
| over | 结束所有网络传输并关闭这些线程 | 无 | 无 |
| is_sending | 查询网络传输状态 | 无 | bool，若正在传输则为 true，否则为 false |
# 例
```
send_video();                      // 创建一个线程传视频
for(int i = 0; i < 114514; i++) {
    CvMat *cam = new_cam();        // 打开摄像头
    Frame mat = get_frame(cam);    // 获取摄像头的一帧图像
    // Copy input Mat
    const uchar *qImageBuffer = (const uchar*)mat.data;
    // Create QImage with same dimensions as input Mat
    QImage img(qImageBuffer, mat.cols, mat.rows, QImage::Format_Indexed8);
    ……
    del_cam(cam);                   // 用完就释放
}
over();                             // 关闭线程停止传视频
```
## 错误示范
```
for(int i = 0; i < 114514; i++) {
    UdpReader *udp = udp_reader();       // 监听 UDP
    Frame mat = udp_frame(udp);          // 获取一帧图像
    // Copy input Mat
    const uchar *qImageBuffer = (const uchar*)mat.data;
    // Create QImage with same dimensions as input Mat
    QImage img(qImageBuffer, mat.cols, mat.rows, QImage::Format_Indexed8);
    ……
}                                        // 循环创建 UdpReader 对象却不及时释放导致内存泄漏
```
## 正确示范
```
UdpReader *udp = udp_reader();           // 循环前创建对象监听 UDP
for(int i = 0; i < 114514; i++) {
    Frame mat = udp_frame(udp);          // 获取一帧图像
    // Copy input Mat
    const uchar *qImageBuffer = (const uchar*)mat.data;
    // Create QImage with same dimensions as input Mat
    QImage img(qImageBuffer, mat.cols, mat.rows, QImage::Format_Indexed8);
    ……
}
del_udp(udp)                             // 释放内存
```
