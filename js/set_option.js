powerColor = ${power} > (maxPower * 0.8) ? 'rgb(255, 70, 131)' : '#58D9F9';
speedColor = ${speed} > (maxSpeed * 0.8) ? 'rgb(255, 70, 131)' : '#58D9F9';
chart.setOption({
    graphic: [
        {},
        {
            style: {
                text: ${heart}
            }
        },
        {},
        {
            style: {
                text: ${cadence},
            }
        },
        {
            style: {
                text: (${grade}).toFixed(2) + '%',
            }
        },
    ],
    series: [
        {},
        {
            data: [[${long},${lat}]]
        },
        {},
        {
            data: [[${timestamp},${alt}]]
        },
        {
            axisTick: {
                lineStyle: {
                    color: speedColor
                },
            },
            splitLine: {
                lineStyle: {
                    color: speedColor
                },
            },
            axisLabel: {
                color: speedColor
            },
            itemStyle: {
                color: speedColor
            },
            detail: {
                color: speedColor
            },
            title: {
                color: speedColor
            },
            data: [
                {
                    value: ${speed},
                    name: 'km/h'
                }
            ]
        },
        {
            axisTick: {
                lineStyle: {
                    color: powerColor
                },
            },
            splitLine: {
                lineStyle: {
                    color: powerColor
                },
            },
            axisLabel: {
                color: powerColor
            },
            itemStyle: {
                color: powerColor
            },
            detail: {
                color: powerColor
            },
            data: [
                {
                    value: ${power}
                }
            ]
        }
    ]
});
chart.renderToSVGString();