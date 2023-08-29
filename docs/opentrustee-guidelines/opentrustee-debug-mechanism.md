# OpenTrustee调试机制介绍

## 日志
当前OpenTrustee支持TEE侧日志系统，在TEEOS中提供LIB接口供TA和OpenTrustee Framework调用记录日志。

### 日志级别

当前TEE中日志分为以下5个级别。

<a name="table263813112481"></a>
<table><thead align="left"><tr id="row763951124815"><th class="cellrowborder" valign="top" width="21.052105210521052%" id="mcps1.1.4.1.1"><p id="p463916111482"><a name="p463916111482"></a><a name="p463916111482"></a>日志级别</p>
</th>
<th class="cellrowborder" valign="top" width="45.614561456145616%" id="mcps1.1.4.1.2"><p id="p46393115489"><a name="p46393115489"></a><a name="p46393115489"></a>说明</p>
</th>
<th class="cellrowborder" valign="top" width="33.33333333333333%" id="mcps1.1.4.1.3"><p id="p46391111204818"><a name="p46391111204818"></a><a name="p46391111204818"></a>对应的日志打印接口</p>
</th>
</tr>
</thead>
<tbody><tr id="row563919117484"><td class="cellrowborder" valign="top" width="21.052105210521052%" headers="mcps1.1.4.1.1 "><p id="p18639511124817"><a name="p18639511124817"></a><a name="p18639511124817"></a>VERBOSE</p>
</td>
<td class="cellrowborder" valign="top" width="45.614561456145616%" headers="mcps1.1.4.1.2 "><p id="p66391511134814"><a name="p66391511134814"></a><a name="p66391511134814"></a>细粒度信息事件，详细地记录程序的运行过程。</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.1.4.1.3 "><p id="p13639161117487"><a name="p13639161117487"></a><a name="p13639161117487"></a>tlogv</p>
</td>
</tr>
<tr id="row10639201112487"><td class="cellrowborder" valign="top" width="21.052105210521052%" headers="mcps1.1.4.1.1 "><p id="p10639111184818"><a name="p10639111184818"></a><a name="p10639111184818"></a>DEBUG</p>
</td>
<td class="cellrowborder" valign="top" width="45.614561456145616%" headers="mcps1.1.4.1.2 "><p id="p7558134616494"><a name="p7558134616494"></a><a name="p7558134616494"></a>细粒度信息事件，用于开发调试。</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.1.4.1.3 "><p id="p46391911164815"><a name="p46391911164815"></a><a name="p46391911164815"></a>tlogd</p>
</td>
</tr>
<tr id="row176395115486"><td class="cellrowborder" valign="top" width="21.052105210521052%" headers="mcps1.1.4.1.1 "><p id="p146391611194810"><a name="p146391611194810"></a><a name="p146391611194810"></a>INFO</p>
</td>
<td class="cellrowborder" valign="top" width="45.614561456145616%" headers="mcps1.1.4.1.2 "><p id="p1063911124815"><a name="p1063911124815"></a><a name="p1063911124815"></a>突出强调应用程序的运行过程。</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.1.4.1.3 "><p id="p1461319875015"><a name="p1461319875015"></a><a name="p1461319875015"></a>tlogi</p>
</td>
</tr>
<tr id="row863981134816"><td class="cellrowborder" valign="top" width="21.052105210521052%" headers="mcps1.1.4.1.1 "><p id="p4639201113486"><a name="p4639201113486"></a><a name="p4639201113486"></a>WARNING</p>
</td>
<td class="cellrowborder" valign="top" width="45.614561456145616%" headers="mcps1.1.4.1.2 "><p id="p7639911174815"><a name="p7639911174815"></a><a name="p7639911174815"></a>潜在的错误事件，不影响系统的继续运行。</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.1.4.1.3 "><p id="p9639111115480"><a name="p9639111115480"></a><a name="p9639111115480"></a>tlogw</p>
</td>
</tr>
<tr id="row1639171113485"><td class="cellrowborder" valign="top" width="21.052105210521052%" headers="mcps1.1.4.1.1 "><p id="p14639151117487"><a name="p14639151117487"></a><a name="p14639151117487"></a>ERROR</p>
</td>
<td class="cellrowborder" valign="top" width="45.614561456145616%" headers="mcps1.1.4.1.2 "><p id="p1125856174915"><a name="p1125856174915"></a><a name="p1125856174915"></a>发生错误事件，影响系统的继续运行。</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.1.4.1.3 "><p id="p1563971113484"><a name="p1563971113484"></a><a name="p1563971113484"></a>tloge</p>
</td>
</tr>
</tbody>
</table>

### 使用方法

在源代码文件中包含tee\_log.h，使用与日志级别对应的日志打印接口打印日志。

日志级别和相应的接口如上面描述，其使用与标准C中的printf类似。

tee\_log.h中默认使用的日志级别为INFO，即默认打印使用tloge、tlogw、tlogi接口的信息。如调试过程中希望打印其他级别，可选择下面方式中的一种进行修改，建议使用方式1：

1.  在Makefile文件中动态修改日志级别TA\_LOG\_LEVEL，具体值和对应的显示级别为：

    <a name="table1136311211099"></a>
    <table><thead align="left"><tr id="row936414211913"><th class="cellrowborder" valign="top" width="50%" id="mcps1.1.3.1.1"><p id="p10364172119913"><a name="p10364172119913"></a><a name="p10364172119913"></a>TA_LOG_LEVEL值</p>
    </th>
    <th class="cellrowborder" valign="top" width="50%" id="mcps1.1.3.1.2"><p id="p1364162113919"><a name="p1364162113919"></a><a name="p1364162113919"></a>日志显示级别</p>
    </th>
    </tr>
    </thead>
    <tbody><tr id="row2036413219915"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.1 "><p id="p93938731415"><a name="p93938731415"></a><a name="p93938731415"></a>0</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.2 "><p id="p1226241161412"><a name="p1226241161412"></a><a name="p1226241161412"></a>Error</p>
    </td>
    </tr>
    <tr id="row5364202112914"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.1 "><p id="p539312714149"><a name="p539312714149"></a><a name="p539312714149"></a>1</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.2 "><p id="p122621511111413"><a name="p122621511111413"></a><a name="p122621511111413"></a>Error、Warn</p>
    </td>
    </tr>
    <tr id="row536413217917"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.1 "><p id="p5393779149"><a name="p5393779149"></a><a name="p5393779149"></a>2--默认值</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.2 "><p id="p1426221181412"><a name="p1426221181412"></a><a name="p1426221181412"></a>Error、Warn、Info</p>
    </td>
    </tr>
    <tr id="row183648211790"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.1 "><p id="p143931576149"><a name="p143931576149"></a><a name="p143931576149"></a>3</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.2 "><p id="p14262191141410"><a name="p14262191141410"></a><a name="p14262191141410"></a>Error、Warn、Info、Debug</p>
    </td>
    </tr>
    <tr id="row236418211891"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.1 "><p id="p3393177121414"><a name="p3393177121414"></a><a name="p3393177121414"></a>&gt;=4</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.2 "><p id="p1526381111143"><a name="p1526381111143"></a><a name="p1526381111143"></a>Error、Warn、Info、Debug、Verbo</p>
    </td>
    </tr>
    </tbody>
    </table>

2.  直接修改tee\_log.h中定义的TA\_LOG\_LEVEL\_DEFAULT为期望的级别。多个TA共用同一个tee\_log.h文件且期望打印的日志级别不一致时，不能选用此方式。
3.  直接修改tee\_log.h中定义的TA\_LOG\_LEVEL为期望的级别。多个TA共用同一个tee\_log.h文件且期望打印的日志级别不一致时，不能选用此方式。

### 日志使用限制

1.  日志内存大小是有限的，当日志写入比较快时，会出现日志被覆盖的情况，日志内存采用的是内核申请的PAGES内存，大小是256K。
2.  增加TA日志时，要符合安全规范，不能打印用户隐私信息、内存地址等，不打印调试日志，且一条日志内容要做到字符精简。

### TEE侧日志查看

通过`hdc shell`打开命令行窗口，输入tlogcat即可查看日志。