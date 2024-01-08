const chartWidth = ${width};
const chartHeight = ${height};

var chart = echarts.init(null, null, {
    renderer: 'svg',
    ssr: true,
    width: chartWidth,
    height: chartHeight
});

function autoFontSize(scale) {
    return Math.round(chartWidth / scale);
}

const data = ${all_record};
const pathData = data.map(item => [item.lo, item.la]);
const altData = data.map(item => [item.t, item.a]);
let maxSpeed = 0;
let maxPower = 0;
let maxAlt = 0;
let minAlt = 8848.43;
for (let i = 0; i < data.length; i++) {
    const item = data[i];
    if (item.p > maxPower) {
        maxPower = item.p;
    }
    if (item.s > maxSpeed) {
        maxSpeed = item.s;
    }
    if (item.a > maxAlt) {
        maxAlt = item.a;
    }
    if (item.a < minAlt) {
        minAlt = item.a;
    }
}

chart.setOption({animation: false});
chart.setOption(
    {
        grid: [
            {
                left: '75%',
                top: '5%',
                width: '25%',
                height: '25%'
            },
            {
                left: '-0.2%',
                top: '75%',
                width: '25%',
                height: '25%'
            }
        ],
        xAxis: [
            {
                type: 'value',
                scale: true,
                name: 'Longitude',
                show: false
            },
            {
                type: 'category',
                data: [],
                gridIndex: 1,
                show: false
            }
        ],
        yAxis: [
            {
                type: 'value',
                scale: true,
                name: 'Latitude',
                show: false,
            },
            {
                type: 'value',
                gridIndex: 1,
                axisLabel: {
                    fontSize: autoFontSize(75),
                    fontWeight: 'bold',
                },
                splitLine: {
                    show: false
                },
                min: minAlt,
                max: maxAlt,
                show: false
            },
        ],
        series: [
            {
                name: 'Path',
                type: 'line',
                data: pathData,
                showSymbol: false,
                animation: false,
                lineStyle: {
                    width: autoFontSize(300),
                    color: 'white'
                }
            },
            {
                name: 'Ball',
                type: 'effectScatter',
                coordinateSystem: 'cartesian2d',
                z: 100,
                data: [pathData[0]],
                symbolSize: autoFontSize(150),
                showEffectOn: 'render',
                rippleEffect: {
                    brushType: 'stroke'
                },
                itemStyle: {
                    color: '#58D9F9'
                }
            },
            {
                data: altData,
                type: 'line',
                itemStyle: {
                    color: 'white'
                },
                areaStyle: {
                    opacity: 0.8,
                    color: 'rgba(255,255,255,0.5)',
                },
                smooth: true,
                animation: false,
                xAxisIndex: 1,
                yAxisIndex: 1,
                showSymbol: false,
            },
            {
                name: 'Altitude Ball',
                type: 'effectScatter',
                coordinateSystem: 'cartesian2d',
                z: 100,
                data: [altData[0]],
                symbolSize: autoFontSize(150),
                showEffectOn: 'render',
                rippleEffect: {
                    brushType: 'stroke'
                },
                itemStyle: {
                    color: 'white'
                },
                xAxisIndex: 1,
                yAxisIndex: 1,
                label: {
                    normal: {
                        show: true,
                        position: 'top',
                        formatter: function (param) {
                            return param.data[1].toFixed(2) + 'm';
                        },
                        textStyle: {
                            color: 'white',
                            fontWeight: 'bold',
                            fontSize: autoFontSize(80)
                        }
                    }
                }
            },
            {
                type: "gauge",
                center: [
                    "90%",
                    "95%"
                ],
                radius: "30%",
                startAngle: 180,
                endAngle: 0,
                min: 0,
                max: maxSpeed,
                splitNumber: 10,
                itemStyle: {
                    color: "#58D9F9",
                    shadowColor: "rgba(0,138,255,0.45)",
                    shadowBlur: 10,
                    shadowOffsetX: 2,
                    shadowOffsetY: 2
                },
                progress: {
                    show: true,
                    roundCap: true,
                    width: autoFontSize(200)
                },
                pointer: {
                    icon: "path://M2090.36389,615.30999 L2090.36389,615.30999 C2091.48372,615.30999 2092.40383,616.194028 2092.44859,617.312956 L2096.90698,728.755929 C2097.05155,732.369577 2094.2393,735.416212 2090.62566,735.56078 C2090.53845,735.564269 2090.45117,735.566014 2090.36389,735.566014 L2090.36389,735.566014 C2086.74736,735.566014 2083.81557,732.63423 2083.81557,729.017692 C2083.81557,728.930412 2083.81732,728.84314 2083.82081,728.755929 L2088.2792,617.312956 C2088.32396,616.194028 2089.24407,615.30999 2090.36389,615.30999 Z",
                    length: "65%",
                    width: autoFontSize(200),
                    offsetCenter: [
                        0,
                        "-20%"
                    ]
                },
                axisLine: {
                    roundCap: true,
                    lineStyle: {
                        width: autoFontSize(200)
                    }
                },
                axisTick: {
                    splitNumber: 2,
                    length: autoFontSize(250),
                    lineStyle: {
                        width: autoFontSize(1000),
                        color: "#fff"
                    }
                },
                splitLine: {
                    length: autoFontSize(125),
                    lineStyle: {
                        width: autoFontSize(750),
                        color: "#fff"
                    }
                },
                axisLabel: {
                    distance: autoFontSize(100),
                    color: "#58D9F9",
                    fontSize: autoFontSize(100),
                    formatter: function (value) {
                        return value.toFixed(0);
                    }
                },
                title: {
                    offsetCenter: [0, '0%'],
                    fontSize: autoFontSize(50),
                    fontWeight: "bolder",
                    color: "#58D9F9",
                },
                detail: {
                    backgroundColor: "#ffffff00",
                    width: "60%",
                    lineHeight: 20,
                    height: 20,
                    offsetCenter: [
                        0,
                        "-20%"
                    ],
                    valueAnimation: true,
                    fontSize: autoFontSize(50),
                    fontWeight: "bolder",
                    color: "#58D9F9",
                    formatter: function (value) {
                        return value.toFixed(2);
                    }
                },
                data: [
                    {
                        value: 0.00,
                        name: 'km/h'
                    }
                ]
            },
            {
                type: 'gauge',
                center: ['50%', '90%'],
                radius: "25%",
                startAngle: 200,
                endAngle: -20,
                min: 0,
                max: maxPower,
                splitNumber: 10,
                itemStyle: {
                    color: '#58D9F9'
                },
                progress: {
                    show: true,
                    width: autoFontSize(192)
                },
                pointer: {
                    show: false
                },
                axisLine: {
                    lineStyle: {
                        width: autoFontSize(192)
                    }
                },
                axisTick: {
                    distance: -autoFontSize(76),
                    splitNumber: autoFontSize(384),
                    lineStyle: {
                        width: autoFontSize(720),
                        color: '#58D9F9'
                    }
                },
                splitLine: {
                    distance: -autoFontSize(64),
                    length: autoFontSize(384),
                    lineStyle: {
                        width: autoFontSize(720),
                        color: '#58D9F9'
                    }
                },
                axisLabel: {
                    distance: -autoFontSize(48),
                    color: '#58D9F9',
                    fontWeight: 'bold',
                    fontSize: autoFontSize(75),
                    formatter: function (value) {
                        return value.toFixed(0);
                    }
                },
                anchor: {
                    show: false
                },
                title: {
                    show: false
                },
                detail: {
                    valueAnimation: true,
                    width: '60%',
                    offsetCenter: [0, '-15%'],
                    fontSize: autoFontSize(32),
                    fontWeight: 'bolder',
                    formatter: '{value}W',
                    color: 'inherit'
                },
                data: [
                    {
                        value: 0
                    }
                ]
            },
        ],
        graphic: [
            {
                type: 'text',
                left: '15px',
                top: chartHeight / 2 - autoFontSize(24) - autoFontSize(75),
                style: {
                    text: 'BPM',
                    fill: 'rgba(255,255,255,0.8)',
                    fontSize: autoFontSize(75),
                    textAlign: 'left',
                    fontWeight: "bolder",
                    textVerticalAlign: 'middle'
                }
            },
            {
                type: 'text',
                left: '10px',
                top: chartHeight / 2 - autoFontSize(24),
                style: {
                    text: '0',
                    fill: 'rgba(255,255,255)',
                    fontSize: autoFontSize(16),
                    textAlign: 'left',
                    textVerticalAlign: 'middle'
                }
            },
            {
                type: 'text',
                left: '15px',
                top: chartHeight / 2 + autoFontSize(24) - autoFontSize(75),
                style: {
                    text: 'RPM',
                    fill: 'rgba(255,255,255,0.8)',
                    fontSize: autoFontSize(75),
                    textAlign: 'left',
                    fontWeight: "bolder",
                    textVerticalAlign: 'middle'
                }
            },
            {
                type: 'text',
                left: '10px',
                top: chartHeight / 2 + autoFontSize(24),
                style: {
                    text: '0',
                    fill: 'rgba(255,255,255)',
                    fontSize: autoFontSize(16),
                    textAlign: 'left',
                    textVerticalAlign: 'middle'
                }
            },
            {
                type: 'text',
                left: autoFontSize(48) + 'px',
                top: chartHeight - autoFontSize(32) - chartHeight / 16,
                style: {
                    text: '0.00%',
                    fill: 'rgba(255,255,255)',
                    fontSize: autoFontSize(32),
                    textAlign: 'left',
                    textVerticalAlign: 'middle'
                }
            },
        ]
    });

"OK"