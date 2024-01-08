# fit-overlay-cli

[English](./README.md)

`fit-overlay-cli` 是一个可以将fit的数据导出为叠加层视频的工具。

## 功能

- 使用 [echarts](https://github.com/apache/echarts)，支持自定义图表，更多图表样式请看：[echarts examples](https://echarts.apache.org/examples/zh/index.html)
- 使用 [FFmpeg](https://github.com/FFmpeg/FFmpeg)，支持硬件加速导出，使用命令`./fit-overlay-cli ffmpeg --args -encoders`查看

## 安装

下载 [fit-overlay-cli](https://github.com/zzyandzzy/fit-overlay-cli/releases) 对应电脑的二进制文件解压就行。

## 使用


1.首先安装[FFmpeg](https://github.com/FFmpeg/FFmpeg)或者运行下面的命令

```shell
# 该命令会首先下载并解压FFmpeg到当前文件夹。
./fit-overlay-cli ffmpeg --args -version
```

2. 运行

```shell
# 根据tests.fit生成一段时间戳大于等于1696483082到1696483112(1696483082+30)秒的视频
# --delay参数表示fit记录的数据和视频记录的时间差距为17秒
./fit-overlay-cli gen --fit-path ./tests/tests.fit --start-timestamp 1696483082 --delay -17 --duration 30 --codec h264

# 查看支持的硬件编码加速器
./fit-overlay-cli ffmpeg --args -encoders | grep h264
Apple M2 output:
 V....D h264_videotoolbox    VideoToolbox H.264 Encoder (codec h264)

# 所以可以用h264_videotoolbox硬件加速器
./fit-overlay-cli gen --fit-path ./tests/tests.fit --start-timestamp 1696483082 --delay -17 --duration 30 --codec h264_videotoolbox
# 或者hevc(h265)
./fit-overlay-cli gen --fit-path ./tests/tests.fit --start-timestamp 1696483082 --delay -17 --duration 30 --codec hevc_videotoolbox

# 更多命令细节请查看
./fit-overlay-cli --help
```

## 许可证

fit-overlay-cli 采用 MIT 许可证

```text
MIT License

Copyright (c) 2020 intent

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## 致谢

- [FFmpeg](https://github.com/FFmpeg/FFmpeg)
- [echarts](https://github.com/apache/echarts)
- [charming](https://github.com/yuankunzhang/charming)

