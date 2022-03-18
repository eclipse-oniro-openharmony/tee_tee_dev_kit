# TEE 安全镜像Loader适配指导<a name="ZH-CN_TOPIC_0000001259645325"></a>

-   [概述](#section28401073463)
    -   [功能简介](#section8714171624613)
    -   [约束与限制](#section1856813710473)

-   [开发指导](#section1890104185118)
    -   [场景介绍](#section114981014165118)
    -   [接口说明](#section125843344514)

-   [接口参数结构体定义](#section15511163311570)
    -   [开发步骤](#section26814557584)
    -   [开发实例](#section3748141455319)


## 概述<a name="section28401073463"></a>

### 功能简介<a name="section8714171624613"></a>

TEE Loader主要负责加载安全镜像并将启动参数传递给TEE OS的功能。

### 约束与限制<a name="section1856813710473"></a>

-   芯片架构为ARMv7/ARMv8架构。
-   CPU需支持安全内存和非安全内存的划分，安全和非安全外设的划分。

## 开发指导<a name="section1890104185118"></a>

### 场景介绍<a name="section114981014165118"></a>

由于安全镜像在flash中是以加密形式存储的，因此需要对镜像进行解密处理，随后拷贝到目标执行地址。在TEE OS启动时，需要对其传递启动参数。以上这些都是在Loader中进行的。因此在芯片使能TEE时，需要对Loader进行开发适配。

### 接口说明<a name="section125843344514"></a>

以下接口说明列表将介绍我们在实际使能TEE过程中对Loader适配将用到的接口。包括：

-   启动参数配置
-   镜像加载
-   共享内存传递

**表 1**  启动参数配置调用接口表

<a name="table1496711447522"></a>
<table><thead align="left"><tr id="row596815449522"><th class="cellrowborder" valign="top" width="47.67%" id="mcps1.2.4.1.1"><p id="p196816442521"><a name="p196816442521"></a><a name="p196816442521"></a>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="32.129999999999995%" id="mcps1.2.4.1.2"><p id="p09681444165214"><a name="p09681444165214"></a><a name="p09681444165214"></a>描述</p>
</th>
<th class="cellrowborder" valign="top" width="20.200000000000003%" id="mcps1.2.4.1.3"><p id="p19431194717249"><a name="p19431194717249"></a><a name="p19431194717249"></a>必选/可选</p>
</th>
</tr>
</thead>
<tbody><tr id="row51478313471"><td class="cellrowborder" valign="top" width="47.67%" headers="mcps1.2.4.1.1 "><p id="p497mcpsimp"><a name="p497mcpsimp"></a><a name="p497mcpsimp"></a>void set_teeos_mem(uintptr_t teeos_base_addr, uint64_t size)</p>
</td>
<td class="cellrowborder" valign="top" width="32.129999999999995%" headers="mcps1.2.4.1.2 "><p id="p498mcpsimp"><a name="p498mcpsimp"></a><a name="p498mcpsimp"></a>设置TEE OS的起始地址size大小。</p>
</td>
<td class="cellrowborder" valign="top" width="20.200000000000003%" headers="mcps1.2.4.1.3 "><p id="p12431947122419"><a name="p12431947122419"></a><a name="p12431947122419"></a><strong id="b1532325172812"><a name="b1532325172812"></a><a name="b1532325172812"></a>必选，OS需要。</strong></p>
</td>
</tr>
<tr id="row214793104714"><td class="cellrowborder" valign="top" width="47.67%" headers="mcps1.2.4.1.1 "><p id="p66112184581"><a name="p66112184581"></a><a name="p66112184581"></a>void set_teeos_uart(uint64_t uart_addr, uint64_t uart_type)</p>
</td>
<td class="cellrowborder" valign="top" width="32.129999999999995%" headers="mcps1.2.4.1.2 "><p id="p196103189585"><a name="p196103189585"></a><a name="p196103189585"></a>设置串口地址和大小。</p>
</td>
<td class="cellrowborder" valign="top" width="20.200000000000003%" headers="mcps1.2.4.1.3 "><p id="p164311747112411"><a name="p164311747112411"></a><a name="p164311747112411"></a>可选，建议配置，启动调试用。</p>
</td>
</tr>
<tr id="row314712319477"><td class="cellrowborder" valign="top" width="47.67%" headers="mcps1.2.4.1.1 "><p id="p503mcpsimp"><a name="p503mcpsimp"></a><a name="p503mcpsimp"></a>bool set_protected_regions(p_region_t *protected_regions, uint32_t regions_num)</p>
</td>
<td class="cellrowborder" valign="top" width="32.129999999999995%" headers="mcps1.2.4.1.2 "><p id="p504mcpsimp"><a name="p504mcpsimp"></a><a name="p504mcpsimp"></a>设置TEE OS内存访问受限的区域。</p>
</td>
<td class="cellrowborder" valign="top" width="20.200000000000003%" headers="mcps1.2.4.1.3 "><p id="p6431347102415"><a name="p6431347102415"></a><a name="p6431347102415"></a><strong id="b513951013281"><a name="b513951013281"></a><a name="b513951013281"></a>必选，安全需要。</strong></p>
</td>
</tr>
<tr id="row15147143144719"><td class="cellrowborder" valign="top" width="47.67%" headers="mcps1.2.4.1.1 "><p id="p88631144105813"><a name="p88631144105813"></a><a name="p88631144105813"></a>void set_sharedmem_range(uint64_t shmem_size)</p>
</td>
<td class="cellrowborder" valign="top" width="32.129999999999995%" headers="mcps1.2.4.1.2 "><p id="p10982102193017"><a name="p10982102193017"></a><a name="p10982102193017"></a>设置共享内存的大小。</p>
</td>
<td class="cellrowborder" valign="top" width="20.200000000000003%" headers="mcps1.2.4.1.3 "><p id="p164311047182410"><a name="p164311047182410"></a><a name="p164311047182410"></a><strong id="b1375812194285"><a name="b1375812194285"></a><a name="b1375812194285"></a>必选，按业务需要配置大小。</strong></p>
</td>
</tr>
<tr id="row151471133477"><td class="cellrowborder" valign="top" width="47.67%" headers="mcps1.2.4.1.1 "><p id="p509mcpsimp"><a name="p509mcpsimp"></a><a name="p509mcpsimp"></a>bool set_random(uint64_t random_data)</p>
</td>
<td class="cellrowborder" valign="top" width="32.129999999999995%" headers="mcps1.2.4.1.2 "><p id="p510mcpsimp"><a name="p510mcpsimp"></a><a name="p510mcpsimp"></a>产生硬件随机数并设置。</p>
</td>
<td class="cellrowborder" valign="top" width="20.200000000000003%" headers="mcps1.2.4.1.3 "><p id="p18431247102412"><a name="p18431247102412"></a><a name="p18431247102412"></a><strong id="b424517234282"><a name="b424517234282"></a><a name="b424517234282"></a>必选，安全需要。</strong></p>
</td>
</tr>
<tr id="row3147173114720"><td class="cellrowborder" valign="top" width="47.67%" headers="mcps1.2.4.1.1 "><p id="p512mcpsimp"><a name="p512mcpsimp"></a><a name="p512mcpsimp"></a>void set_gic(struct gic_config_t gic_config)</p>
</td>
<td class="cellrowborder" valign="top" width="32.129999999999995%" headers="mcps1.2.4.1.2 "><p id="p513mcpsimp"><a name="p513mcpsimp"></a><a name="p513mcpsimp"></a>配置gic寄存器。结构体定义见。</p>
</td>
<td class="cellrowborder" valign="top" width="20.200000000000003%" headers="mcps1.2.4.1.3 "><p id="p1043194716242"><a name="p1043194716242"></a><a name="p1043194716242"></a><strong id="b0722234132818"><a name="b0722234132818"></a><a name="b0722234132818"></a>必选，OS需要。</strong></p>
</td>
</tr>
<tr id="row201471034473"><td class="cellrowborder" valign="top" width="47.67%" headers="mcps1.2.4.1.1 "><p id="p514mcpsimp"><a name="p514mcpsimp"></a><a name="p514mcpsimp"></a>void set_spi_num(uint32_t spi_num)</p>
</td>
<td class="cellrowborder" valign="top" width="32.129999999999995%" headers="mcps1.2.4.1.2 "><p id="p515mcpsimp"><a name="p515mcpsimp"></a><a name="p515mcpsimp"></a>配置spi类型的中断号给TEE OS使用。</p>
</td>
<td class="cellrowborder" valign="top" width="20.200000000000003%" headers="mcps1.2.4.1.3 "><p id="p18431184772417"><a name="p18431184772417"></a><a name="p18431184772417"></a><strong id="b241338192814"><a name="b241338192814"></a><a name="b241338192814"></a>必选，OS需要。</strong></p>
</td>
</tr>
<tr id="row314712320476"><td class="cellrowborder" valign="top" width="47.67%" headers="mcps1.2.4.1.1 "><p id="p516mcpsimp"><a name="p516mcpsimp"></a><a name="p516mcpsimp"></a>void set_plat_features(uint64_t plat_features)</p>
</td>
<td class="cellrowborder" valign="top" width="32.129999999999995%" headers="mcps1.2.4.1.2 "><p id="p517mcpsimp"><a name="p517mcpsimp"></a><a name="p517mcpsimp"></a>设置芯片属性。</p>
</td>
<td class="cellrowborder" valign="top" width="20.200000000000003%" headers="mcps1.2.4.1.3 "><p id="p14310471243"><a name="p14310471243"></a><a name="p14310471243"></a><strong id="b838114262813"><a name="b838114262813"></a><a name="b838114262813"></a>必选，OS需要。</strong></p>
</td>
</tr>
<tr id="row914714313476"><td class="cellrowborder" valign="top" width="47.67%" headers="mcps1.2.4.1.1 "><p id="p2744165514582"><a name="p2744165514582"></a><a name="p2744165514582"></a>bool copy_extend_datas(void * extend_datas, uint64_t extend_length)</p>
</td>
<td class="cellrowborder" valign="top" width="32.129999999999995%" headers="mcps1.2.4.1.2 "><p id="p519mcpsimp"><a name="p519mcpsimp"></a><a name="p519mcpsimp"></a>保留字段拷贝。</p>
</td>
<td class="cellrowborder" valign="top" width="20.200000000000003%" headers="mcps1.2.4.1.3 "><p id="p1431194762415"><a name="p1431194762415"></a><a name="p1431194762415"></a>可选，根据实际情况。</p>
</td>
</tr>
<tr id="row514715384711"><td class="cellrowborder" valign="top" width="47.67%" headers="mcps1.2.4.1.1 "><p id="p520mcpsimp"><a name="p520mcpsimp"></a><a name="p520mcpsimp"></a>bool copy_teeos_cfg(void)</p>
</td>
<td class="cellrowborder" valign="top" width="32.129999999999995%" headers="mcps1.2.4.1.2 "><p id="p521mcpsimp"><a name="p521mcpsimp"></a><a name="p521mcpsimp"></a>拷贝启动参数到目标位置，<strong id="b163513559278"><a name="b163513559278"></a><a name="b163513559278"></a>配置TEE OS属性后再调用此接口</strong>。</p>
</td>
<td class="cellrowborder" valign="top" width="20.200000000000003%" headers="mcps1.2.4.1.3 "><p id="p3431747192413"><a name="p3431747192413"></a><a name="p3431747192413"></a><strong id="b625310456284"><a name="b625310456284"></a><a name="b625310456284"></a>必选，OS需要。</strong></p>
</td>
</tr>
</tbody>
</table>

**表 2**  镜像加载调用接口表

<a name="table15516113715563"></a>
<table><thead align="left"><tr id="row5516113795615"><th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.1"><p id="p10101114295614"><a name="p10101114295614"></a><a name="p10101114295614"></a>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.2"><p id="p161017421562"><a name="p161017421562"></a><a name="p161017421562"></a>描述</p>
</th>
</tr>
</thead>
<tbody><tr id="row14516237105613"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p1336862755413"><a name="p1336862755413"></a><a name="p1336862755413"></a>uintptr_t read_teeos(const char *part_name, uint32_t part_size)</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p1351643714569"><a name="p1351643714569"></a><a name="p1351643714569"></a>将TEE OS从flash分区读镜像到RAM中。</p>
</td>
</tr>
<tr id="row051633710563"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p1463764075419"><a name="p1463764075419"></a><a name="p1463764075419"></a>int32_t verify_teeos(uintptr_t buf_addr)</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p105166372562"><a name="p105166372562"></a><a name="p105166372562"></a>TEE OS镜像验签。</p>
</td>
</tr>
<tr id="row15516143712565"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p155165377567"><a name="p155165377567"></a><a name="p155165377567"></a>int32_t decrypt_teeos(uintptr_t buf_addr)</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p851619377562"><a name="p851619377562"></a><a name="p851619377562"></a>TEE OS镜像解密。</p>
</td>
</tr>
<tr id="row13516153714567"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p351653735618"><a name="p351653735618"></a><a name="p351653735618"></a>int32_t copy_teeos(uintptr_t buf_addr)</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p152771458171114"><a name="p152771458171114"></a><a name="p152771458171114"></a>TEE OS镜像拷贝到目标执行地址。</p>
</td>
</tr>
<tr id="row15835727135513"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p158111828105519"><a name="p158111828105519"></a><a name="p158111828105519"></a>uint64_t get_teeos_start(void)</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p281114283558"><a name="p281114283558"></a><a name="p281114283558"></a>获取TEE OS启动地址。</p>
</td>
</tr>
</tbody>
</table>

**表 3**  共享内存传递调用接口表

<a name="table1245194216594"></a>
<table><thead align="left"><tr id="row192451742185916"><th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.1"><p id="p926941619017"><a name="p926941619017"></a><a name="p926941619017"></a>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.2"><p id="p2269101615015"><a name="p2269101615015"></a><a name="p2269101615015"></a>描述</p>
</th>
</tr>
</thead>
<tbody><tr id="row7344114631012"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p1556855901018"><a name="p1556855901018"></a><a name="p1556855901018"></a>int32_t trans_oemkey(uint8_t *oemkey, uint32_t oemkey_size)</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p19568125951012"><a name="p19568125951012"></a><a name="p19568125951012"></a>传递oemkey给TEE OS。</p>
</td>
</tr>
<tr id="row105151750201019"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p1056905912108"><a name="p1056905912108"></a><a name="p1056905912108"></a>uint32_t put_tlv_shared_mem(struct tlv_item_data tlv_item_data)</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p115691759141011"><a name="p115691759141011"></a><a name="p115691759141011"></a>将value中的内容转换为tlv编码格式拷贝至shared memory中。结构体定义见<a href="#section15511163311570">接口参数结构体定义</a>。</p>
</td>
</tr>
<tr id="row12354448131013"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p356935901017"><a name="p356935901017"></a><a name="p356935901017"></a>uint32_t append_share_mem_tlv(struct tlv_item_data tlv_item_data)</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p115691459121014"><a name="p115691459121014"></a><a name="p115691459121014"></a>更新之前写入的tlv的内容。</p>
</td>
</tr>
<tr id="row588119440105"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p1569659181010"><a name="p1569659181010"></a><a name="p1569659181010"></a>int32_t tee_get_shared_mem(const char *type, uint32_t type_size, void *buffer, uint32_t *buffer_size, bool clear_flag)</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p65692059171019"><a name="p65692059171019"></a><a name="p65692059171019"></a>获取loader写入的tlv格式的共享内存。</p>
</td>
</tr>
</tbody>
</table>

## 接口参数结构体定义<a name="section15511163311570"></a>

```
struct gic_config_t {
    char version;
    union {
    struct v2_t {
        p_region_t dist;
        p_region_t contr;
    } v2;
    struct v3_t {
        p_region_t dist;
        uint32_t redist_num;
        uint32_t redist_stride;
        p_region_t redist[GICR_MAX_NUM];
        } v3;
   };
};

struct tlv_item_data {
    char *type;
    uint32_t type_len;
    void *owner_list;
    uint32_t owner_len;
    void *value;
    uint32_t value_len;
} __attribute__((__packed__));
```

### 开发步骤<a name="section26814557584"></a>

1.  启动参数配置

    启动参数包含TEE OS用到的内存地址和大小，串口的地址和类型，共享内存的偏移和大小，硬件随机数，gic寄存器配置，spi中断号配置，芯片属性配置，其它拓展参数。

    loader中启动参数的适配方法可以按照产品的习 惯要求采用不同的方法，例如结构体中直接填写相应的参数，或者增加配置文件的方法。

    **表 4**  启动参数列表

    <a name="table31859499130"></a>
    <table><thead align="left"><tr id="row3185174941311"><th class="cellrowborder" valign="top" width="20.31%" id="mcps1.2.4.1.1"><p id="p9186154916134"><a name="p9186154916134"></a><a name="p9186154916134"></a>参数</p>
    </th>
    <th class="cellrowborder" valign="top" width="28.64%" id="mcps1.2.4.1.2"><p id="p16186104911131"><a name="p16186104911131"></a><a name="p16186104911131"></a>含义</p>
    </th>
    <th class="cellrowborder" valign="top" width="51.05%" id="mcps1.2.4.1.3"><p id="p12200103644211"><a name="p12200103644211"></a><a name="p12200103644211"></a>备注</p>
    </th>
    </tr>
    </thead>
    <tbody><tr id="row20186849151316"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p5186194911131"><a name="p5186194911131"></a><a name="p5186194911131"></a>plat_cfg_size</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p618617496136"><a name="p618617496136"></a><a name="p618617496136"></a>启动参数大小，包含extend_paras部分</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p1520073664218"><a name="p1520073664218"></a><a name="p1520073664218"></a>整个启动参数的buffer大小，值为sizeof（struct platform_info） + extend _length。</p>
    </td>
    </tr>
    <tr id="row1618618496132"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p292mcpsimp"><a name="p292mcpsimp"></a><a name="p292mcpsimp"></a>phys_region_size</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p294mcpsimp"><a name="p294mcpsimp"></a><a name="p294mcpsimp"></a>TEE OS内存大小</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p320093615420"><a name="p320093615420"></a><a name="p320093615420"></a>2MB的整数倍。</p>
    </td>
    </tr>
    <tr id="row01861049151317"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p297mcpsimp"><a name="p297mcpsimp"></a><a name="p297mcpsimp"></a>phys_region_start</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p299mcpsimp"><a name="p299mcpsimp"></a><a name="p299mcpsimp"></a>TEE OS内存起始物理地址</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p1020063614220"><a name="p1020063614220"></a><a name="p1020063614220"></a>2MB的整数倍。</p>
    </td>
    </tr>
    <tr id="row18186649111319"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p302mcpsimp"><a name="p302mcpsimp"></a><a name="p302mcpsimp"></a>uart_addr</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p304mcpsimp"><a name="p304mcpsimp"></a><a name="p304mcpsimp"></a>串口寄存器的基地址</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p1920015362427"><a name="p1920015362427"></a><a name="p1920015362427"></a>-</p>
    </td>
    </tr>
    <tr id="row121861849191315"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p307mcpsimp"><a name="p307mcpsimp"></a><a name="p307mcpsimp"></a>uart_type</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p309mcpsimp"><a name="p309mcpsimp"></a><a name="p309mcpsimp"></a>串口类型</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p2353201518528"><a name="p2353201518528"></a><a name="p2353201518528"></a>串口是否使能标志和串口打印类型做“或”运算。即（UART_ENABLE_FLAG | type_id）或者（UART_DISABLE_FLAG | type_id）。</p>
    </td>
    </tr>
    <tr id="row11186149201315"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p5244914147"><a name="p5244914147"></a><a name="p5244914147"></a>protected_regions</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p121114931416"><a name="p121114931416"></a><a name="p121114931416"></a>访问受限区域</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p18200336114212"><a name="p18200336114212"></a><a name="p18200336114212"></a>存放TEE OS中不允许映射的地址空间，如ATF使用的内存和TZASC寄存器。</p>
    </td>
    </tr>
    <tr id="row71863499132"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p6154931418"><a name="p6154931418"></a><a name="p6154931418"></a>shmem_reserved</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p16995174841414"><a name="p16995174841414"></a><a name="p16995174841414"></a>参数预留</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p120023694217"><a name="p120023694217"></a><a name="p120023694217"></a>-</p>
    </td>
    </tr>
    <tr id="row34291626124617"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p1842914268465"><a name="p1842914268465"></a><a name="p1842914268465"></a>shmem_size</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p144299269467"><a name="p144299269467"></a><a name="p144299269467"></a>共享内存大小</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p3429126164615"><a name="p3429126164615"></a><a name="p3429126164615"></a>-</p>
    </td>
    </tr>
    <tr id="row351051719464"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p351011171461"><a name="p351011171461"></a><a name="p351011171461"></a>random_seed</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p3510111784616"><a name="p3510111784616"></a><a name="p3510111784616"></a>随机数</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p1400168155116"><a name="p1400168155116"></a><a name="p1400168155116"></a>loader需要传递一个硬件随机数（不能为0）给TEE OS，仅在TEE OS的启动过程中使用，需要且仅需要一个硬件随机数。</p>
    </td>
    </tr>
    <tr id="row59731131174610"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p9973113119460"><a name="p9973113119460"></a><a name="p9973113119460"></a>struct gic_config_t</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p11973103114611"><a name="p11973103114611"></a><a name="p11973103114611"></a>gic寄存器参数</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p797319312466"><a name="p797319312466"></a><a name="p797319312466"></a>-</p>
    </td>
    </tr>
    <tr id="row6720133444611"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p57201341465"><a name="p57201341465"></a><a name="p57201341465"></a>spi_num_for_notify</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p77209343464"><a name="p77209343464"></a><a name="p77209343464"></a>SPI中断号</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p27201034194614"><a name="p27201034194614"></a><a name="p27201034194614"></a>需要分配一个SPI类型的中断号（不能与任何外设绑定）给TEE OS使用，用于TEE OS主动发消息通知REE侧。</p>
    </td>
    </tr>
    <tr id="row1746216191466"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p1346210195466"><a name="p1346210195466"></a><a name="p1346210195466"></a>plat_features</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p546214196461"><a name="p546214196461"></a><a name="p546214196461"></a>芯片属性</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p1477803417512"><a name="p1477803417512"></a><a name="p1477803417512"></a>在具有PAN特性（ARM v8.2及以后的特性）的芯片上，plat_features赋值为PLAT_ENABLE_PAN。</p>
    </td>
    </tr>
    <tr id="row1188221104618"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p19884213469"><a name="p19884213469"></a><a name="p19884213469"></a>extend_datas</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p98811215461"><a name="p98811215461"></a><a name="p98811215461"></a>其它参数</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p18872120461"><a name="p18872120461"></a><a name="p18872120461"></a>格式需要和TEE OS核对。</p>
    </td>
    </tr>
    </tbody>
    </table>

2.  镜像加载

    tee loader适配在芯片平台的bios或者fastboot、preloader中，主要完成的功能有配置并传递TEE OS的启动参数、将TEE OS镜像加载到目标内存位置、传递共享内存信息给TEE OS。loader中启动参数的适配方法可以按照产品的习惯和要求采用不同的方法，例如结构体中直接填写相应的参数，或者增加配置文件。以下为镜像加载步骤：

    1.  从flash分区读镜像到RAM中。
    2.  TEE OS镜像验签。
    3.  TEE OS镜像解密。
    4.  TEE OS镜像拷贝到目标执行地址。
    5.  TEE OS启动地址传递。获取TEE OS的启动地址，一般情况下需要将这个值传给atf, 作为ATF初始化TEE OS的入口地址。

3.  共享内存传递

    这里的共享内存指的是loader与TEE OS协商好的一块内存，便于loader阶段将boot获取到的信息传递给TEE OS。由于目前TEE OS只支持16字节的oemkey，要求oemkey的buffer大小oemkey\_size的值必须为16。为了灵活地支持共享内存的读写，使用tlv的格式对共享内存进行编码，同时引入owner的选项，便于对共享内存的访问进行权限控制。TA的解密依赖oemkey，要求平台适配者必须通过共享内存将tlv传递给TEE OS。

    共享内存传递的内容:

    -   oemkey：如果oemkey的实现不在硬件加解密引擎里面实现，需要通过共享内存传递16字节的oemkey，tag为“oemkey”，owner控制为只允许系统服务访问。
    -   平台芯片信息：格式是“芯片厂商\_平台名称”，例如”hisilicon\_kirin990”，需要通过共享内存传递，tag为”platform\_info”，owner控制为所有TA都能访问。


相关配置接口见[接口说明](#section125843344514)。

### 开发实例<a name="section3748141455319"></a>

在芯片使能TEE时，配置启动参数的开发实例。

```
#define TEEOS_TEXT_OFFSET (0x8000)
#define ALIGN_SIZE_2M    (0x200000)
struct platform_info {
    uint64_t plat_cfg_size;
    uint64_t phys_region_size;
    paddr_t phys_region_start;
    paddr_t uart_addr;
    uint64_t uart_type;
    p_region_t protected_regions[MAX_PROTECTED_REGIONS];
    uint64_t reserved;
    uint64_t shmem_size;
    uint32_t random_seed;
    struct gic_config_t gic_config;
    uint32_t spi_num_for_notify;
    uint64_t plat_features;
    struct extend_datas_t {
        uint64_t extend_length;
        char extend_paras[0];
    } extend_datas;
};
static struct platform_info g_teeos_cfg;

static uintptr_t g_teeos_base_addr = 0;

/* 设置TEE OS的其实地址size的大小，OS需要，必选*/
int32_t set_teeos_mem(uintptr_t teeos_base_addr, uint64_t size)
{
    g_teeos_base_addr = teeos_base_addr;
    g_teeos_cfg.phys_region_size = size;
    if ((uint64_t)teeos_base_addr % ALIGN_SIZE_2M != 0)
        return -1;

    g_teeos_cfg.phys_region_start = (uint64_t)teeos_base_addr;

    return 0;
}

void set_teeos_uart(uint64_t uart_addr, uint64_t uart_type)
{
    g_teeos_cfg.uart_addr = uart_addr;
    g_teeos_cfg.uart_type = uart_type;
}

/* 设置TEE OS内存访问受限的区域，安全需要，必选*/
bool set_protected_regions(p_region_t *protected_regions, uint32_t regions_num)
{
    if (regions_num > MAX_PROTECTED_REGIONS)
        return false;
    if (protected_regions == NULL)
        return false;
    if (memcpy_s(g_teeos_cfg.protected_regions, sizeof(p_region_t) * regions_num,
                 protected_regions, sizeof(p_region_t) * regions_num) != EOK)
        return false;
    return true;
}

void set_sharedmem_size(uint64_t shmem_size)
{
    g_teeos_cfg.shmem_size   = shmem_size;
}

/* 产生随机数并设置，安全需要，必选*/
bool set_random(uint64_t random_data)
{
    if (random_data == 0)
        return false;

    g_teeos_cfg.random_seed = random_data;

    return true;
}

/* 配置gic寄存器*/
void set_gic(struct gic_config_t gic_config)
{
    g_teeos_cfg.gic_config = gic_config;
}

/* 配置spi类型的终端号，OS需要，必选*/
void set_spi_num(uint32_t spi_num)
{
    g_teeos_cfg.spi_num_for_notify = spi_num;
}

/* 配置芯片属性，OS需要，必选*/
void set_plat_features(uint64_t plat_features)
{
    g_teeos_cfg.plat_features = plat_features;
}

bool copy_extend_datas(void *extend_datas, uint64_t extend_length)
{
    if (extend_datas == NULL)
        return false;

    if (sizeof(struct platform_info) + extend_length > MAX_CONFIG_LENGTH)
        return false;
 
    g_teeos_cfg.extend_datas.extend_length = extend_length;
    char *dst = (char *)(uintptr_t)(g_teeos_cfg.phys_region_start + sizeof(g_teeos_cfg));

    if (memcpy_s(dst, MAX_CONFIG_LENGTH - sizeof(g_teeos_cfg),
                 extend_datas, extend_length) != EOK)
        return false;
    return true;
}

/* 拷贝启动参数到目标位置，配置TEE OS属性后调用此接口，OS需要，必选*/
bool copy_teeos_cfg(void)
{
    if (g_teeos_cfg.phys_region_start == 0)
        return false;

    g_teeos_cfg.plat_cfg_size = sizeof(struct platform_info) + g_teeos_cfg.extend_datas.extend_length;
    char *dst = (void *)(uintptr_t)g_teeos_cfg.phys_region_start;

    if (memcpy_s(dst, sizeof(g_teeos_cfg),
                 (char *)&g_teeos_cfg, g_teeos_cfg.plat_cfg_size - sizeof(uint64_t)) != EOK)
        return false;

    return true;
}

uint64_t get_teeos_start(void)
{
    return g_teeos_cfg.phys_region_start;
}

uint64_t get_teeos_code_start(void)
{
    return g_teeos_cfg.phys_region_start + TEEOS_TEXT_OFFSET;
}

uint64_t get_teeos_size(void)
{
    return g_teeos_cfg.phys_region_size;
}

uint64_t get_sharedmem_start(void)
{
    return g_teeos_cfg.phys_region_start + g_teeos_cfg.phys_region_size - g_teeos_cfg.shmem_size;
}

uint64_t get_sharedmem_size(void)
{
    return g_teeos_cfg.shmem_size;
}
```

