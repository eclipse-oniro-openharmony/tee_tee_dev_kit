# TEE 安全驱动开发指导<a name="ZH-CN_TOPIC_0000001212014566"></a>

## 概述<a name="section351681313544"></a>

### 功能简介<a name="section167701178548"></a>

本文对TEE安全驱动的开发流程、接口、函数库等进行说明，指导驱动开发者进行安全驱动程序的开发与调试工作。

### 约束与限制<a name="section67447370556"></a>

-   开发的驱动程序仅支持在TEE可信执行环境子系统上加载和运行。

### 场景介绍<a name="section148731215195617"></a>

目前TEEOS支持内置驱动的开发，驱动和TEEOS镜像共同编译打包。具体开发指导请参考开发示例章节。

## 驱动开发框架<a name="section12432132218372"></a>

驱动开发整体可以分为两部分，第一个是驱动业务开发框架，第二个是访问该驱动的驱动访问者框架。

### 驱动业务框架<a name="section6705194712373"></a>

为方便各个驱动开发统一，TEE可信执行环境子系统为各个驱动设计了一套基础框架，如下所示，驱动开发者结合驱动实际逻辑分别定义这些基础函数即可。
```
#define DRV_NAME_MAX_LEN 32U
#define DRV_RESERVED_NUM 8U

struct drv_data {
    int32_t fd; /* unique label which alloced by driver framework */
    uint32_t taskid; /* caller taskid */
    void *private_data; /* the private data associated with this fd */
    struct tee_uuid uuid; /* caller uuid */
};

typedef int32_t (*init_func)(void);

typedef int32_t (*suspned_func)(void);
typedef int32_t (*resume_func)(void);

typedef int64_t (*ioctl_func)(struct drv_data *drv, uint32_t cmd, unsigned long args, uint32_t args_len);
typedef int64_t (*open_func)(struct drv_data *drv, unsigned long args, uint32_t args_len);
typedef int64_t (*close_func)(struct drv_data *drv);

struct tee_driver_module {
    init_func init;
    ioctl_func ioctl;
    open_func open;
    close_func close;
    suspned_func suspend;
    resume_func resume;
    suspned_func suspend_s4;
    resume_func resume_s4;
    uint64_t reserved[DRV_RESERVED_NUM]; /* has not used, just reserved */
};

#define tee_driver_declare(name, init, open, ioctl, close, suspend, resume, suspend_s4, resume_s4) \
__attribute__((visibility("default"))) const struct tee_driver_module g_driver_##name = { \
    init, ioctl, open, close, suspend, resume, suspend_s4, resume_s4, {0} }
```

结构体struct tee_driver_module便是每个驱动业务开发时需要注册的信息，各个变量说明如下：

**表 1** 驱动业务开发注册信息表

<a></a>
<table><thead align="left"><tr><th class="cellrowborder" valign="top" width="33.33333333333333%"><p>变量名</p>
</th>
<th class="cellrowborder" valign="top" width="33.33333333333333%"><p>类型</p>
</th>
<th class="cellrowborder" valign="top" width="33.33333333333333%"><p>说明</p>
</th>
</tr>
</thead>
<tbody><tr><td><p>name</p></td>
<td><p>常量字符串</p></td>
<td><p>驱动名字，每个驱动必须唯一，有效长度小于32个字节，仅支持数字、字母和'_'。</p></td>
</tr>
<tr><td><p>init</p></td>
<td><p>init_func函数指针</p></td>
<td><p>驱动加载时初始化函数。</p></td>
</tr>
<tr><td><p>open</p></td>
<td><p>函数指针</p></td>
<td><p>驱动被访问时初始化函数。</p></td>
</tr>
<tr><td><p>ioctl</p></td>
<td><p>函数指针</p></td>
<td><p>驱动被访问时命令分发函数。</p></td>
</tr>
<tr><td><p>close</p></td>
<td><p>函数指针</p></td>
<td><p>驱动被访问结束时资源释放函数。</p></td>
</tr>
<tr><td><p>suspend</p></td>
<td><p>函数指针</p></td>
<td><p>驱动在系统休眠时被调用的函数。</p></td>
</tr>
<tr><td><p>resume</p></td>
<td><p>函数指针</p></td>
<td><p>驱动在系统唤醒时被调用的函数。</p></td>
</tr>
<tr><td><p>suspend_s4</p></td>
<td><p>函数指针</p></td>
<td><p>驱动在系统休眠时被调用的函数，对应linux kernel里freeze_noirq操作流程。</p></td>
</tr>
<tr><td><p>resume_s4</p></td>
<td><p>函数指针</p></td>
<td><p>驱动在系统唤醒时被调用的函数，对应linux kernel里restore_noirq操作流程。</p></td>
</tr>
<tr><td><p>reserved</p></td>
<td><p>预留</p></td>
<td><p>现有框架暂未使用。</p></td>
</tr>
</tbody>
</table>

**表 2** 驱动业务框架接口说明
<a></a>
<table><thead><tr><th class="cellrowborder" valign="top" width="50%"><p>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="50%"><p>描述</p>
</th>
</tr>
</thead>
<tbody><tr><td><p>int32_t (*init_func)(void)</p></td>
<td><p>该函数是在驱动加载完后便被驱动框架调用的初始化函数，其主要作用是在该驱动被访问之前进行初始化操作。该函数在驱动加载后的整个生命周期内只会被调用一次。</p>
<p><b>返回值：</b></p>
<p>0：初始化成功</p>
<p>非0：初始化失败</p></td>
</tr>
<tr><td><p>int64_t (*open_func)(struct drv_data *drv, unsigned long args, uint32_t args_len)</p></td>
<td><p>该函数是驱动访问者访问驱动时调用的初始化函数，其主要作用是在驱动里申请一个fd，并进行初始化。该函数在每次新增驱动访问时都会调用。</p>
<p><b>参数：</b></p>
<p>drv：入参，表示此次open该驱动的fd所有信息</p>
<p>args：入参，表示给驱动传入参数对应的buffer地址，由驱动访问者设置</p>
<p>args_len：入参，表示给驱动传入参数对应的buffer长度，由驱动访问者设置</p>
<p><b>返回值：</b></p>
<p>非正数：异常值</p>
<p>其他：返回的fd信息</p></td>
</tr>
<tr><td><p>int64_t (*ioctl_func)(struct drv_data *drv, uint32_t cmd, unsigned long args, uint32_t args_len)</p></td>
<td><p>该函数是驱动在open初始化之后，进行的一系列针对业务逻辑的操作。其主要作用是根据传入的drv获取fd资源信息，再执行cmd命令对应的执行流，其中[args.args_len]组成的buffer信息是给该cmd执行流传入的参数。</p>
<p><b>参数：</b></p>
<p>drv：入参，表示此次ioctl该驱动的fd所有信息，fd由驱动访问者传入，驱动框架根据fd传入对应drv_data结构体</p>
<p>cmd：入参，表示此次ioctl该驱动传入的命令号，驱动可根据不同命令号执行不同的业务逻辑，由驱动访问者设置</p>
<p>args：入参，表示给驱动传递参数的buffer基地址</p>
<p>args_len：入参，表示给驱动传递参数的buffer长度</p>
<p><b>返回值：</b></p>
<p>0：操作成功</p>
<p>-1：操作失败，框架层面返回的异常值</p>
<p>其他值：操作失败，驱动自行定义的异常值</p></td>
</tr>
<tr><td><p>int64_t (*close_func)(struct drv_data *drv)</p></td>
<td><p>该函数主要作用是在驱动业务逻辑访问结束后进行对该fd对应的资源清理操作。</p>
<p><b>参数：</b></p>
<p>drv：入参，表示此次ioctl该驱动的fd所有信息</p>
<p><b>返回值：</b></p>
<p>0：操作成功</p>
<p>非0：操作失败</p></td>
</tr>
<tr><td><p>int32_t (*suspned_func)(void)</p></td>
<td><p>该函数主要作用是此驱动休眠状态下的一些列操作，会在系统休眠时有驱动框架自行调用。</p>
<p><b>返回值：</b></p>
<p>0：操作成功</p>
<p>非0：操作失败</p></td>
</tr>
<tr><td><p>int32_t (*resume_func)(void)</p></td>
<td><p>该函数主要作用是此驱动唤醒态下的一系列操作，会在系统唤醒流程中由驱动框架自行调用，与suspend函数对应。</p>
<p><b>返回值：</b></p>
<p>0：操作成功</p>
<p>非0：操作失败</p></td>
</tr>
</tbody>
</table>

### 驱动访问者框架<a name="section18123151133811"></a>

驱动访问时，驱动访问者先调用tee_drv_open函数获取驱动唯一标记fd；再调用tee_drv_ioctl函数，传入cmd信息，访问该驱动对应cmd执行流，如果针对某个fd有多个ioctl执行流，多次调用tee_drv_ioctl即可；如果访问结束，驱动访问者还需要调用tee_drv_close函数关闭该fd信息。

**表 3** 驱动访问者框架接口说明

<a></a>
<table><thead><tr><th class="cellrowborder" valign="top" width="50%"><p>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="50%"><p>描述</p>
</th>
</tr>
</thead>
<tbody><tr><td><p>int64_t tee_drv_open(const char *drv_name, const void *param, uint32_t param_len)</p></td>
<td><p>主要作用是驱动访问者通过调用该函数，访问drv_name指定的驱动，调用驱动的open函数，返回与该驱动对应的唯一标记fd信息。其中param buffer对应的内容组装结构由drv_name对应驱动定义，实际与驱动open函数里[args.args_len]表示的buffer内容一致。</p>
<p><b>参数：</b></p>
<p>drv_name：入参，表示要访问的驱动名称；</p>
<p>param：入参，表示给驱动传递的参数地址；</p>
<p>param_len：入参，表示给驱动传递的参数长度，与param组成的buffer内容便是给驱动传递的参数信息。</p>
<p><b>返回值：</b></p>
<p>非正数：非法值，操作失败</p>
<p>大于0：fd信息，对应驱动的唯一标识</p></td>
</tr>
<tr><td><p>int64_t tee_drv_ioctl(int64_t fd, uint32_t cmd_id, const void *param, uint32_t param_len)</p></td>
<td><p>主要作用是驱动访问者通过调用该函数，访问fd对应的驱动模块，执行命令ID号为cmd_id对应的业务逻辑，传入参数为param与param_len对应buffer存储的内容，其中param与param_len对应buffer内容组装结构由fd对应的驱动定义，实际与驱动ioctl函数[args.args_len]组成的buffer内容一致。</p>
<p><b>参数：</b></p>
<p>fd：入参，表示open该驱动成功返回时的返回值；</p>
<p>cmd_id：入参，表示ioctl该驱动时对应的命令id号；</p>
<p>param：入参，表示ioctl该驱动cmd_id流程时传入的参数基地址；</p>
<p>param_len：入参，表示ioctl该驱动cmd_id流程时传入的参数buffer长度。与param组成的buffer内容便是给驱动命令号cmd_id对应的参数信息。</p>
<p><b>返回值：</b></p>
<p>0：操作成功</p>
<p>-1：框架层面访问失败</p>
<p>其他值：各驱动自定义失败返回值</p></td>
</tr>
<tr><td><p>int64_t tee_drv_close(int64_t fd)</p></td>
<td><p>主要作用是关闭fd对应驱动信息，一般常见操作是释放该fd维护的驱动资源。</p>
<p><b>返回值：</b></p>
<p>0：操作成功</p>
<p>-1：操作失败</p></td>
</tr>
</tbody>
</table>

## 接口说明<a name="section1748173625619"></a>

### 地址转换接口说明<a name="section101297155716"></a>

驱动进行地址转换操作需要使用的接口列表。

**表 4**  地址转换接口列表

<a name="table13739184111427"></a>
<table><thead align="left"><tr id="row2739154118421"><th class="cellrowborder" valign="top" width="54.31%" id="mcps1.2.3.1.1"><p id="p462103214410"><a name="p462103214410"></a><a name="p462103214410"></a>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="45.69%" id="mcps1.2.3.1.2"><p id="p1762163219448"><a name="p1762163219448"></a><a name="p1762163219448"></a>描述</p>
</th>
</tr>
</thead>
<tbody><tr id="row7739441104213"><td class="cellrowborder" valign="top" width="54.31%" headers="mcps1.2.3.1.1 "><p id="p137395418427"><a name="p137395418427"></a><a name="p137395418427"></a>uint64_t drv_virt_to_phys(uintptr_t addr);</p>
</td>
<td class="cellrowborder" valign="top" width="45.69%" headers="mcps1.2.3.1.2 "><p id="p177391841124218"><a name="p177391841124218"></a><a name="p177391841124218"></a>虚拟地址转换为物理地址。</p>
</td>
</tr>
</tbody>
</table>

### map接口说明<a name="section1831091675716"></a>

驱动进行内存映射操作所需要的接口列表。

**表 5**  map接口列表

<a name="table1690431511432"></a>
<table><thead align="left"><tr id="row189041315194317"><th class="cellrowborder" valign="top" width="54.67999999999999%" id="mcps1.2.3.1.1"><p id="p34281633124413"><a name="p34281633124413"></a><a name="p34281633124413"></a>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="45.32%" id="mcps1.2.3.1.2"><p id="p4428133174412"><a name="p4428133174412"></a><a name="p4428133174412"></a>描述</p>
</th>
</tr>
</thead>
<tbody><tr id="row9904121594312"><td class="cellrowborder" valign="top" width="54.67999999999999%" headers="mcps1.2.3.1.1 "><p id="p1090471594316"><a name="p1090471594316"></a><a name="p1090471594316"></a>int32_t tee_map_secure(paddr_t paddr, uint64_t size, uintptr_t *vaddr, cache_mode_type cache_mode);</p>
</td>
<td class="cellrowborder" valign="top" width="45.32%" headers="mcps1.2.3.1.2 "><p id="p583mcpsimp"><a name="p583mcpsimp"></a><a name="p583mcpsimp"></a>给驱动访问者映射一段安全属性的物理内存。</p>
</td>
</tr>
<tr id="row190417158431"><td class="cellrowborder" valign="top" width="54.67999999999999%" headers="mcps1.2.3.1.1 "><p id="p4904715174317"><a name="p4904715174317"></a><a name="p4904715174317"></a>int32_t tee_map_nonsecure(paddr_t paddr, uint64_t size, uintptr_t *vaddr, cache_mode_type cache_mode);</p>
</td>
<td class="cellrowborder" valign="top" width="45.32%" headers="mcps1.2.3.1.2 "><p id="p10904115204310"><a name="p10904115204310"></a><a name="p10904115204310"></a>给驱动访问者映射一段非安全属性的物理内存，其中映射属性是只读不能写。</p>
</td>
</tr>
</tbody>
</table>

### IO操作接口说明<a name="section6512133012574"></a>

驱动进行IO操作所需要的接口列表。

**表 6**  IO操作接口列表

<a name="table6311346194315"></a>
<table><thead align="left"><tr id="row831212462439"><th class="cellrowborder" valign="top" width="54.730000000000004%" id="mcps1.2.3.1.1"><p id="p7945143412442"><a name="p7945143412442"></a><a name="p7945143412442"></a>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="45.269999999999996%" id="mcps1.2.3.1.2"><p id="p7945183494416"><a name="p7945183494416"></a><a name="p7945183494416"></a>描述</p>
</th>
</tr>
</thead>
<tbody><tr id="row143121846164318"><td class="cellrowborder" valign="top" width="54.730000000000004%" headers="mcps1.2.3.1.1 "><p id="p133121246194317"><a name="p133121246194317"></a><a name="p133121246194317"></a>void *ioremap(uintptr_t phys_addr, unsigned long size, int32_t prot);</p>
</td>
<td class="cellrowborder" valign="top" width="45.269999999999996%" headers="mcps1.2.3.1.2 "><p id="p133126466437"><a name="p133126466437"></a><a name="p133126466437"></a>将IO地址映射至虚拟地址。</p>
</td>
</tr>
<tr id="row1431214468430"><td class="cellrowborder" valign="top" width="54.730000000000004%" headers="mcps1.2.3.1.1 "><p id="p103122046114317"><a name="p103122046114317"></a><a name="p103122046114317"></a>int32_t iounmap(uintptr_t pddr, void *addr);</p>
</td>
<td class="cellrowborder" valign="top" width="45.269999999999996%" headers="mcps1.2.3.1.2 "><p id="p4312194674313"><a name="p4312194674313"></a><a name="p4312194674313"></a>解除物理地址映射。</p>
</td>
</tr>
<tr id="row1131211467433"><td class="cellrowborder" valign="top" width="54.730000000000004%" headers="mcps1.2.3.1.1 "><p id="p63121446154315"><a name="p63121446154315"></a><a name="p63121446154315"></a>void read_from_io (void *to, const volatile void *from, unsigned long count);</p>
</td>
<td class="cellrowborder" valign="top" width="45.269999999999996%" headers="mcps1.2.3.1.2 "><p id="p16312174619438"><a name="p16312174619438"></a><a name="p16312174619438"></a>将IO输入的值读取至驱动指定的地址，读取长度由count指定。</p>
</td>
</tr>
<tr id="row1331219468434"><td class="cellrowborder" valign="top" width="54.730000000000004%" headers="mcps1.2.3.1.1 "><p id="p8312104614437"><a name="p8312104614437"></a><a name="p8312104614437"></a>void write_to_io(volatile void *to, const void *from, unsigned long count);</p>
</td>
<td class="cellrowborder" valign="top" width="45.269999999999996%" headers="mcps1.2.3.1.2 "><p id="p193mcpsimp"><a name="p193mcpsimp"></a><a name="p193mcpsimp"></a>将驱动指定地址的值输出至IO，读取长度由count指定。</p>
</td>
</tr>
</tbody>
</table>

### 内存拷贝接口说明<a name="section75381736318"></a>

驱动进行内存拷贝操作所需要的接口列表。

**表 7**  内存拷贝接口列表

<a name="table7469133214314"></a>
<table><thead align="left"><tr id="row24693321135"><th class="cellrowborder" valign="top" width="55.35%" id="mcps1.2.3.1.1"><p id="p109015511039"><a name="p109015511039"></a><a name="p109015511039"></a>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="44.65%" id="mcps1.2.3.1.2"><p id="p2090117510315"><a name="p2090117510315"></a><a name="p2090117510315"></a>描述</p>
</th>
</tr>
</thead>
<tbody><tr id="row15469133212312"><td class="cellrowborder" valign="top" width="55.35%" headers="mcps1.2.3.1.1 "><p id="p04692032734"><a name="p04692032734"></a><a name="p04692032734"></a>int32_t copy_from_client(uint64_t src, uint32_t src_size, uintptr_t dst, uint32_t dst_size);</p>
</td>
<td class="cellrowborder" valign="top" width="44.65%" headers="mcps1.2.3.1.2 "><p id="p184693323311"><a name="p184693323311"></a><a name="p184693323311"></a>将驱动内存拷入client端。</p>
</td>
</tr>
<tr id="row246915325319"><td class="cellrowborder" valign="top" width="55.35%" headers="mcps1.2.3.1.1 "><p id="p94881980512"><a name="p94881980512"></a><a name="p94881980512"></a>int32_t copy_to_client(uintptr_t src, uint32_t src_size, uint64_t dst, uint32_t dst_size);</p>
</td>
<td class="cellrowborder" valign="top" width="44.65%" headers="mcps1.2.3.1.2 "><p id="p647018327314"><a name="p647018327314"></a><a name="p647018327314"></a>将client端数据拷出至驱动。</p>
</td>
</tr>
</tbody>
</table>

### 共享内存接口说明<a name="section5891654459"></a>

驱动进行共享内存操作所需要的接口列表。

**表 8**  共享内存接口列表

<a name="table186909151063"></a>
<table><thead align="left"><tr id="row369016151263"><th class="cellrowborder" valign="top" width="55.620000000000005%" id="mcps1.2.3.1.1"><p id="p1033416207618"><a name="p1033416207618"></a><a name="p1033416207618"></a>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="44.379999999999995%" id="mcps1.2.3.1.2"><p id="p19334172010619"><a name="p19334172010619"></a><a name="p19334172010619"></a>描述</p>
</th>
</tr>
</thead>
<tbody><tr id="row1069019153610"><td class="cellrowborder" valign="top" width="55.620000000000005%" headers="mcps1.2.3.1.1 "><p id="p176901815266"><a name="p176901815266"></a><a name="p176901815266"></a>void *tee_alloc_sharemem_aux(const struct tee_uuid *uuid, uint32_t size);</p>
</td>
<td class="cellrowborder" valign="top" width="44.379999999999995%" headers="mcps1.2.3.1.2 "><p id="p11690101520612"><a name="p11690101520612"></a><a name="p11690101520612"></a>申请进程间通信共享内存。</p>
</td>
</tr>
<tr id="row19690615967"><td class="cellrowborder" valign="top" width="55.620000000000005%" headers="mcps1.2.3.1.1 "><p id="p166901915464"><a name="p166901915464"></a><a name="p166901915464"></a>uint32_t tee_free_sharemem(void *addr, uint32_t size);</p>
</td>
<td class="cellrowborder" valign="top" width="44.379999999999995%" headers="mcps1.2.3.1.2 "><p id="p186901215366"><a name="p186901215366"></a><a name="p186901215366"></a>释放进程间通信共享内存。</p>
</td>
</tr>
</tbody>
</table>

## 开发示例<a name="section198361520981"></a>
### 编译文件配置
- 进入base/tee/tee_os_framework/目录
- 首先在drivers/目录新建一个目录，例如demo_driver
- 在demo_driver目录新建src目录用于存放源代码文件，新建Makefile编译文件

Makefile示例如下

```
DRIVER := demo_driver.elf

include $(BUILD_CONFIG)/var.mk

demo_driver_c_files += $(wildcard src/*.c) 

#include
inc-flags += -I./src

# Libraries

SVC_PARTITIAL_LINK = y
include $(BUILD_SERVICE)/svc-common.mk
```

- 修改build/mk/common/operation/project.mk，增加drivers += demo_driver
- 修改build/mk/pack/plat_config.mk, 增加以下内容

```
ifeq ($(CONFIG_ARCH_AARCH64),y)
product_apps += $(OUTPUTDIR)/aarch64/drivers/demo_driver.elf
check-a64-syms-y += $(OUTPUTDIR)/aarch64/drivers/demo_driver.elf
else
product_apps += $(OUTPUTDIR)/arm/drivers/demo_driver.elf
check-syms-y += $(OUTPUTDIR)/arm/drivers/demo_driver.elf
endif
```

### 驱动开发示例<a name="section198361520981"></a>

驱动框架注册实例如下：

```
#define TEST_NUM_INIT_VAL 0x11
#define TEST_NUM_ADD_TAG 0x10
static int32_t g_test_num = TEST_NUM_INIT_VAL;

/* 驱动初始化函数 */
int32_t init_test(void)
{
    if (g_test_num != TEST_NUM_INIT_VAL) {
        tloge("driver init test failed, g_test_num:0x%x not equal 0x%x\n", g_test_num, TEST_NUM_INIT_VAL);
        return -1;
    }

    g_test_num += TEST_NUM_ADD_TAG;
    tloge("driver init test end\n");
    return 0;
}

static int32_t buf_check(uint32_t *buf, uint32_t size, uint32_t args)
{
    if (buf == NULL) {
        tloge("buf is invalid, check failed\n");
        return -1;
    }

    uint32_t i;
    for (i = 0; i < size; i++) {
        if (buf[i] != args) {
            tloge("buf[%u]=%u which not equal %u\n", i, buf[i], args);
            return -1;
        }
    }

    return 0;
}

/* 驱动具体业务处理逻辑函数 */
static int64_t args_test(struct drv_data *drv, unsigned long args, uint32_t args_len)
{
    /* cmd处理函数在使用参数之前需要判断参数合法性 */
    if (args_len != sizeof(uint32_t) || args == 0) {
        tloge("invalid args args_len:%u\n", args_len);
        return -1;
    }

    uint32_t *input_args = (uint32_t *)(uintptr_t)args;

    tloge("driver args test begin: drv.fd=%d args=0x%x g_test_num:0x%x\n", drv->fd, *input_args, g_test_num);
    int64_t ret = buf_check(drv->private_data, TOKEN_BUF_SIZE, *input_args);
    if (ret != 0)
        tloge("args test FAIL\n");
    else
        tloge("args test SUCC\n");

    return ret;
}

/* 驱动命令分发函数，根据不同cmdid调用不同函数逻辑 */
int64_t ioctl_test(struct drv_data *drv, uint32_t cmd, unsigned long args, uint32_t args_len)
{
    if (drv == NULL) { /* drv可以在调用业务逻辑之前判断，args和args_len需要根据具体cmdid逻辑去判断 */
        tloge("ioctl invalid drv\n");
        return -1;
    }

    int64_t ret = -1;

    switch (cmd) {
    case ARGS_TEST_ID:
        ret = args_test(drv, args, args_len);
        break;
    default:
        tloge("cmd:0x%x not support\n", cmd);
    }

    return ret;
}

static uint32_t *buf_init(uint32_t args)
{
    uint32_t *buf = (uint32_t *)malloc(TOKEN_BUF_SIZE * sizeof(uint32_t));
    if (buf == NULL) {
        tloge("alloc buf failed\n");
        return NULL;
    }

    int32_t i;
    for (i = 0; i < TOKEN_BUF_SIZE; i++)
        buf[i] = args;

    return buf;
}

int64_t open_test(struct drv_data *drv, unsigned long args, uint32_t args_len)
{
    if (drv == NULL) {
        tloge("open invalid drv\n");
        return -1;
    }

    if (args_len < sizeof(uint32_t) || args == 0) {
        tloge("open invalid drv\n");
        return -1;
    }

    uint32_t *input = (uint32_t *)args;
    if (*input == UINT32_MAX) {
        tloge("open test input args is UINT32_MAX, just retrun -1\n");
        return -1;
    }

    uint32_t *buf = buf_init(*input);
    if (buf == NULL)
        return -1;

    /* 将buf赋值给fd结构的private_data，后续都可以通过此fd获取到buf内容 */
    drv->private_data = buf;
    tloge("driver open test begin: fd=%d args=0x%x g_test_num:0x%x\n", drv->fd, *input, g_test_num);

    return 0;
}

int64_t close_test(struct drv_data *drv)
{
    if (drv == NULL) {
        tloge("close invalid drv\n");
        return -1;
    }

    tloge("driver close test begin: fd:%d g_test_num:0x%x\n", drv->fd, g_test_num);
    /* close流程需要释放fd对应的所有资源 */
    if (drv->private_data != NULL) {
        tloge("free private data in close\n");
        free(drv->private_data);
    }

    return 0;
}

int32_t suspend_test(void)
{
    tloge("suspend test begin\n");
    return 0;
}

int32_t resume_test(void)
{
    tloge("resume test begin\n");
    return 0;
}

int32_t suspend_s4_test(void)
{
    tloge("suspend_s4 test begin\n");
    return 0;
}

int32_t resume_s4_test(void)
{
    tloge("resume_s4 test begin\n");
    return 0;
}

/* 驱动框架注册 */
tee_driver_declare(drv_test_module, init_test, open_test, ioctl_test, close_test, \
                   suspend_test, resume_test, suspend_s4_test, resume_s4_test);
```

### 驱动访问者开发示例<a name="section198361520981"></a>
```

#include <securec.h>
#include <stdlib.h>
#include <string.h>
#include <tee_drv_client.h>
#include <tee_ext_api.h>
#include <tee_log.h>
#include <test_drv_cmdid.h>
#include <mem_ops.h>

#define CA_PKGN_VENDOR "/vendor/bin/tee_test_drv"
#define CA_PKGN_SYSTEM "/system/bin/tee_test_drv"
#define CA_UID 0

#define DRV_UUID1                                          \
    {                                                      \
        0x11112222, 0x0000, 0x0000,                        \
        {                                                  \
            0x00, 0x00, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11 \
        }                                                  \
    }

#define BUFFER_SIZE 1024
struct share_buffer_arg {
    uint64_t addr;
    uint32_t len;
    uint32_t share_token;
};

static TEE_Result TeeTestDrive(uint32_t cmd)
{
    int ret;
    const char *drvName = "drv_test_module";
    uint32_t args = (uint32_t)(&drvName);
    const char drvcallerInput[] = "the param is drvcaller_input";
    char drvOutput[] = "DRVMEM_OUTPUT";

    uint32_t drvcallerInputLen = (uint32_t)strlen(drvcallerInput) + 1;
    uint32_t drvOutputLen = (uint32_t)strlen(drvOutput) + 1;
    TEE_UUID uuid = DRV_UUID1;

    /* 调用驱动的open函数，返回与该驱动对应的唯一标记fd信息 */
    int64_t fd = tee_drv_open(drvName, &args, sizeof(args));
    if (fd <= 0) {
        tloge("open %s for get fd failed\n", drvName);
        return TEE_ERROR_GENERIC;
    }

    char *tempBuffer = tee_alloc_sharemem_aux(&uuid, BUFFER_SIZE);
    if (tempBuffer == NULL) {
        tloge("alloc share mem failed\n");
        return TEE_ERROR_GENERIC;
    }
    (void)memset_s(tempBuffer, BUFFER_SIZE, 0x0, BUFFER_SIZE);
    ret = strcpy_s(tempBuffer, drvcallerInputLen, drvcallerInput);
    if (ret != 0) {
        tloge("strcpy_s failed,ret = 0x%x\n", ret);
        return TEE_ERROR_GENERIC;
    }

    struct share_buffer_arg inputArg = { 0 };
#ifndef __aarch64__
    inputArg.addr = (uint64_t)(uint32_t)tempBuffer;
#else
    inputArg.addr = (uint64_t)tempBuffer;
#endif

    inputArg.len = BUFFER_SIZE;

    tlogi("%s drv test ioctl begin args:0x%x fd:%d\n", drvName, inputArg, (int32_t)fd);

    /* 访问fd对应的驱动模块，执行命令ID号为cmd_id对应的业务逻辑 */
    ret = (int)tee_drv_ioctl(fd, cmd, (const void *)(&inputArg), sizeof(inputArg));
    if (ret != 0) {
        tloge("%s drv test ioctl failed, fd:%d \n", drvName, (int32_t)fd);
    }
    if (cmd == DRVTEST_COMMAND_COPYTOCLIENT) {
        if (strncmp(drvOutput, (char *)tempBuffer, drvOutputLen) != 0) {
            tloge("%s drv copy_to_client test failed, fd:%d, heap_buffer is:%s \n", drvName, (int32_t)fd, tempBuffer);
            free_sharemem(tempBuffer, BUFFER_SIZE);
            return TEE_ERROR_GENERIC;
        }
    }

    /* 关闭fd对应驱动信息 */
    ret |= (int)tee_drv_close(fd);
    if (ret != 0) {
        tloge("drv test fail!\n");
    }

    if (free_sharemem(tempBuffer, BUFFER_SIZE) != 0) {
        tloge("free sharemem failed\n");
        ret = -1;
    }
    return (TEE_Result)ret;
}

TEE_Result TA_CreateEntryPoint(void)
{
    tlogi("---- TA_CreateEntryPoint ----------- \n");
    TEE_Result ret;

    ret = AddCaller_CA_exec(CA_PKGN_VENDOR, CA_UID);
    if (ret != TEE_SUCCESS) {
        tloge("add caller failed, ret: 0x%x", ret);
        return ret;
    }

    ret = AddCaller_CA_exec(CA_PKGN_SYSTEM, CA_UID);
    if (ret != TEE_SUCCESS) {
        tloge("add caller failed, ret: 0x%x", ret);
        return ret;
    }

    return TEE_SUCCESS;
}

TEE_Result TA_OpenSessionEntryPoint(uint32_t parmType, TEE_Param params[4], void **sessionContext)
{
    (void)parmType;
    (void)sessionContext;
    tlogi("---- TA_OpenSessionEntryPoint -------- \n");
    if (params[0].value.b == 0xFFFFFFFE)
        return TEE_ERROR_GENERIC;
    else
        return TEE_SUCCESS;
}

TEE_Result TA_InvokeCommandEntryPoint(void *sessionContext, uint32_t cmd, uint32_t parmType, TEE_Param params[4])
{
    TEE_Result ret = TEE_SUCCESS;
    (void)sessionContext;
    (void)parmType;
    (void)params;
    tlogi("---- TA invoke command ----------- command id: %u\n", cmd);

    switch (cmd) {
        case DRVTEST_COMMAND_DRVVIRTTOPHYS:
        case DRVTEST_COMMAND_COPYFROMCLIENT:
        case DRVTEST_COMMAND_COPYTOCLIENT:
            ret = TeeTestDrive(cmd);
            if (ret != TEE_SUCCESS)
                tloge("invoke command for driver test failed! cmdId: %u, ret: 0x%x\n", cmd, ret);
            break;
        default:
            tloge("not support this invoke command! cmdId: %u\n", cmd);
            ret = TEE_ERROR_GENERIC;
            break;
    }

    return ret;
}

void TA_CloseSessionEntryPoint(void *sessionContext)
{
    (void)sessionContext;
    tlogi("---- TA_CloseSessionEntryPoint ----- \n");
}

void TA_DestroyEntryPoint(void)
{
    tlogi("---- TA_DestroyEntryPoint ---- \n");
}
```

## 标准C库支持<a name="section14109139161012"></a>

支持绝大多数的libc接口。支持大多数的POSIX接口，具体支持情况请看附件。标准文档请参照如下网址：

POSIX:[https://mirror.math.princeton.edu/pub/oldlinux/download/c953.pdf](https://mirror.math.princeton.edu/pub/oldlinux/download/c953.pdf)

目前使用的openharmony的musl-1.2.5/libc库。

**表 9**  标准C支持列表

<a name="table7336617112614"></a>
<table><thead align="left"><tr id="row1633681714266"><th class="cellrowborder" valign="top" width="33.33333333333333%" id="mcps1.2.4.1.1"><p id="p1653154512717"><a name="p1653154512717"></a><a name="p1653154512717"></a>模块</p>
</th>
<th class="cellrowborder" valign="top" width="33.33333333333333%" id="mcps1.2.4.1.2"><p id="p1253114512274"><a name="p1253114512274"></a><a name="p1253114512274"></a>函数接口名</p>
</th>
<th class="cellrowborder" valign="top" width="33.33333333333333%" id="mcps1.2.4.1.3"><p id="p75311145202718"><a name="p75311145202718"></a><a name="p75311145202718"></a>说明</p>
</th>
</tr>
</thead>
<tbody><tr id="row6340121710262"><td class="cellrowborder" rowspan="6" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p18534845102711"><a name="p18534845102711"></a><a name="p18534845102711"></a>malloc</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p553444592710"><a name="p553444592710"></a><a name="p553444592710"></a>aligned_alloc</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">-</td>
</tr>
<tr id="row10341181782614"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p753424552719"><a name="p753424552719"></a><a name="p753424552719"></a>calloc</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row534110172268"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p1853444542719"><a name="p1853444542719"></a><a name="p1853444542719"></a>malloc</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row203411176269"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p1553474572712"><a name="p1553474572712"></a><a name="p1553474572712"></a>realloc</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row5341131742616"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p10534184519273"><a name="p10534184519273"></a><a name="p10534184519273"></a>free</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>posix_memalign</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row18341121732619"><td class="cellrowborder" rowspan="2" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p125341945182720"><a name="p125341945182720"></a><a name="p125341945182720"></a>mman</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p7534184532717"><a name="p7534184532717"></a><a name="p7534184532717"></a>mmap</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">-</td>
</tr>
<tr id="row534131772614"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p12534184513273"><a name="p12534184513273"></a><a name="p12534184513273"></a>munmap</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row434101742618"><td class="cellrowborder" rowspan="3" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p253419457279"><a name="p253419457279"></a><a name="p253419457279"></a>time</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p16534124510274"><a name="p16534124510274"></a><a name="p16534124510274"></a>gettimeofday</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">-</td>
</tr>
<tr id="row53411917172619"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p11534124522712"><a name="p11534124522712"></a><a name="p11534124522712"></a>strftime</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row5341101712617"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p19535845152713"><a name="p19535845152713"></a><a name="p19535845152713"></a>time</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row1334131792612"><td class="cellrowborder" rowspan="6" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p105357458277"><a name="p105357458277"></a><a name="p105357458277"></a>stdio</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p75354454278"><a name="p75354454278"></a><a name="p75354454278"></a>printf</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 "><p id="p4535134517270"><a name="p4535134517270"></a><a name="p4535134517270"></a>目前不支持文件系统，文件操作只支持标准输入输出。</p>
</td>
</tr>
<tr id="row63421517192616"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p2535154592716"><a name="p2535154592716"></a><a name="p2535154592716"></a>scanf</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row83421817132613"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p105351045132712"><a name="p105351045132712"></a><a name="p105351045132712"></a>snprintf</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row434291782619"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p12535545132715"><a name="p12535545132715"></a><a name="p12535545132715"></a>sprintf</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row19342517122616"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p3535245132711"><a name="p3535245132711"></a><a name="p3535245132711"></a>vsnprintf</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row1342161792613"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p353518454270"><a name="p353518454270"></a><a name="p353518454270"></a>vsprintf</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row734291715264"><td class="cellrowborder" rowspan="2" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p13535045172712"><a name="p13535045172712"></a><a name="p13535045172712"></a>errno</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p25351645132719"><a name="p25351645132719"></a><a name="p25351645132719"></a>errno</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">-</td>
</tr>
<tr id="row11342181711269"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p11535164515276"><a name="p11535164515276"></a><a name="p11535164515276"></a>strerror</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row5342151710264"><td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p10535945152714"><a name="p10535945152714"></a><a name="p10535945152714"></a>exit</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p8535124519274"><a name="p8535124519274"></a><a name="p8535124519274"></a>abort</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">-</td>
</tr>
<tr id="row11342117182611"><td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p85358458277"><a name="p85358458277"></a><a name="p85358458277"></a>unistd</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p1753517452273"><a name="p1753517452273"></a><a name="p1753517452273"></a>getpid</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">-</td>
</tr>
<tr id="row1034281772615"><td class="cellrowborder" rowspan="4" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p1553514592714"><a name="p1553514592714"></a><a name="p1553514592714"></a>locale</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p753574512278"><a name="p753574512278"></a><a name="p753574512278"></a>setlocale</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">-</td>
</tr>
<tr id="row334220171263"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p115353451276"><a name="p115353451276"></a><a name="p115353451276"></a>strcoll</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row13343151762618"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p95351145142716"><a name="p95351145142716"></a><a name="p95351145142716"></a>strxfrm</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row173433172262"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p0535104512273"><a name="p0535104512273"></a><a name="p0535104512273"></a>strtod</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row2343131732614"><td class="cellrowborder" rowspan="3" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p95351945132716"><a name="p95351945132716"></a><a name="p95351945132716"></a>multibyte</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p25351345122715"><a name="p25351345122715"></a><a name="p25351345122715"></a>mbrtowc</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">-</td>
</tr>
<tr id="row1234321772614"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p185354453273"><a name="p185354453273"></a><a name="p185354453273"></a>wcrtomb</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row7343151762613"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p115361445112716"><a name="p115361445112716"></a><a name="p115361445112716"></a>wctob</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row1034351714262"><td class="cellrowborder" rowspan="4" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p135361645112710"><a name="p135361645112710"></a><a name="p135361645112710"></a>prng</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p19536134517272"><a name="p19536134517272"></a><a name="p19536134517272"></a>srandom</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">-</td>
</tr>
<tr id="row73431717102610"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p17536134518272"><a name="p17536134518272"></a><a name="p17536134518272"></a>initstate</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row17343181742615"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p2536144515271"><a name="p2536144515271"></a><a name="p2536144515271"></a>setstate</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row16343121711263"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p185362459278"><a name="p185362459278"></a><a name="p185362459278"></a>random</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row63431217102612"><td class="cellrowborder" rowspan="17" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p1153618451278"><a name="p1153618451278"></a><a name="p1153618451278"></a>string</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p1553620456277"><a name="p1553620456277"></a><a name="p1553620456277"></a>memchr</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">-</td>
</tr>
<tr id="row93431217112613"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p14536154510275"><a name="p14536154510275"></a><a name="p14536154510275"></a>memcmp</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row17344417112619"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p153684515275"><a name="p153684515275"></a><a name="p153684515275"></a>memcpy</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row1934421712265"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p1653694592712"><a name="p1653694592712"></a><a name="p1653694592712"></a>memmove</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row934411712267"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p2536114510274"><a name="p2536114510274"></a><a name="p2536114510274"></a>memset</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row163441117142619"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p105361345112715"><a name="p105361345112715"></a><a name="p105361345112715"></a>strchr</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row1434451715262"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p175366451279"><a name="p175366451279"></a><a name="p175366451279"></a>strcmp</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row534471762616"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p353694514273"><a name="p353694514273"></a><a name="p353694514273"></a>strcpy</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row334461752611"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p1753694515274"><a name="p1753694515274"></a><a name="p1753694515274"></a>strlen</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row834419172265"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p0536745102720"><a name="p0536745102720"></a><a name="p0536745102720"></a>strncmp</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row1734415175268"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p55361345192717"><a name="p55361345192717"></a><a name="p55361345192717"></a>strncpy</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row183445174264"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p115365457273"><a name="p115365457273"></a><a name="p115365457273"></a>strnlen</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row1534401712611"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p19537245112712"><a name="p19537245112712"></a><a name="p19537245112712"></a>strrchr</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row13451617142620"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p125371456272"><a name="p125371456272"></a><a name="p125371456272"></a>strstr</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row15345181762614"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p95371845142718"><a name="p95371845142718"></a><a name="p95371845142718"></a>wcschr</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row10345151718267"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p0537124512279"><a name="p0537124512279"></a><a name="p0537124512279"></a>wcslen</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row2345111710263"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p145371645142718"><a name="p145371645142718"></a><a name="p145371645142718"></a>wmemchr</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row12345131715263"><td class="cellrowborder" rowspan="13" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p753714502717"><a name="p753714502717"></a><a name="p753714502717"></a>ctype</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p5537145142720"><a name="p5537145142720"></a><a name="p5537145142720"></a>isalpha</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">-</td>
</tr>
<tr id="row1434581792614"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p0537184511274"><a name="p0537184511274"></a><a name="p0537184511274"></a>isascii</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row5345417182620"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p8537114513278"><a name="p8537114513278"></a><a name="p8537114513278"></a>isdigit</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row8345117122618"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p8537204582712"><a name="p8537204582712"></a><a name="p8537204582712"></a>islower</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row034551782617"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p253716455273"><a name="p253716455273"></a><a name="p253716455273"></a>isprint</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row4345101713267"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p1053713455279"><a name="p1053713455279"></a><a name="p1053713455279"></a>isspace</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row16345201752616"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p853764562716"><a name="p853764562716"></a><a name="p853764562716"></a>iswctype</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row33455179264"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p18537114522715"><a name="p18537114522715"></a><a name="p18537114522715"></a>iswdigit</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row83461317192614"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p16537154512710"><a name="p16537154512710"></a><a name="p16537154512710"></a>iswlower</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row93461117152620"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p353713458276"><a name="p353713458276"></a><a name="p353713458276"></a>iswspace</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row19346161711268"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p75371145202717"><a name="p75371145202717"></a><a name="p75371145202717"></a>iswupper</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row123461517162613"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p3537245152710"><a name="p3537245152710"></a><a name="p3537245152710"></a>towupper</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row1734671714268"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p175381245172715"><a name="p175381245172715"></a><a name="p175381245172715"></a>towlower</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row6346117112615"><td class="cellrowborder" rowspan="16" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p2538174512714"><a name="p2538174512714"></a><a name="p2538174512714"></a>math</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p19538184572712"><a name="p19538184572712"></a><a name="p19538184572712"></a>atan</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">-</td>
</tr>
<tr id="row8346131782613"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p15381945172717"><a name="p15381945172717"></a><a name="p15381945172717"></a>ceil</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row734617176261"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p953844582716"><a name="p953844582716"></a><a name="p953844582716"></a>ceilf</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row134610174264"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p053824511271"><a name="p053824511271"></a><a name="p053824511271"></a>copysignl</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row13346917152610"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p13538104522717"><a name="p13538104522717"></a><a name="p13538104522717"></a>exp</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row334661712269"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p5538154512710"><a name="p5538154512710"></a><a name="p5538154512710"></a>fabs</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row2347181722616"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p2538184522716"><a name="p2538184522716"></a><a name="p2538184522716"></a>floor</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row123471517122616"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p1453874562717"><a name="p1453874562717"></a><a name="p1453874562717"></a>frexp</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row153471917112616"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p14538134512271"><a name="p14538134512271"></a><a name="p14538134512271"></a>frexpl</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row234771717261"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p10538245102714"><a name="p10538245102714"></a><a name="p10538245102714"></a>log</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row1334711179265"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p185381245142719"><a name="p185381245142719"></a><a name="p185381245142719"></a>log2</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row103471317132612"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p20538144512275"><a name="p20538144512275"></a><a name="p20538144512275"></a>pow</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row133472178262"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p35389459277"><a name="p35389459277"></a><a name="p35389459277"></a>roundf</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row234731713261"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p4538154522715"><a name="p4538154522715"></a><a name="p4538154522715"></a>scalbn</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row1347917182619"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p85381845192718"><a name="p85381845192718"></a><a name="p85381845192718"></a>scalbnl</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row1134717178267"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p11538845102718"><a name="p11538845102718"></a><a name="p11538845102718"></a>sqrt</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row13348101715261"><td class="cellrowborder" rowspan="14" align="left" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p195391445102713"><a name="p195391445102713"></a><a name="p195391445102713"></a>stdlib</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p1153954552713"><a name="p1153954552713"></a><a name="p1153954552713"></a>abs</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">-</td>
</tr>
<tr id="row4348101732615"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p1853910450272"><a name="p1853910450272"></a><a name="p1853910450272"></a>atof</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row12348111782617"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p12539124562717"><a name="p12539124562717"></a><a name="p12539124562717"></a>atoi</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row123482178261"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p95391945102713"><a name="p95391945102713"></a><a name="p95391945102713"></a>atol</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row163481117142610"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p35391745172718"><a name="p35391745172718"></a><a name="p35391745172718"></a>atoll</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row6348217132613"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p4539164511276"><a name="p4539164511276"></a><a name="p4539164511276"></a>bsearch</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row12348191714269"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p195391245102717"><a name="p195391245102717"></a><a name="p195391245102717"></a>div</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row1634810170260"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p10539124513274"><a name="p10539124513274"></a><a name="p10539124513274"></a>ecvt</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row103481517182617"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p75392457277"><a name="p75392457277"></a><a name="p75392457277"></a>imaxabs</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row1834817175260"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p854014552719"><a name="p854014552719"></a><a name="p854014552719"></a>llabs</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row1349121792612"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p1154054562718"><a name="p1154054562718"></a><a name="p1154054562718"></a>qsort</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row15349117192614"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p14540645172719"><a name="p14540645172719"></a><a name="p14540645172719"></a>strtoul</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row134910176267"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p135401445172712"><a name="p135401445172712"></a><a name="p135401445172712"></a>strtol</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
<tr id="row183494173268"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p3540114519272"><a name="p3540114519272"></a><a name="p3540114519272"></a>wcstod</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">-</td>
</tr>
</tbody>
</table>

>![](public_sys-resources/icon-caution.gif) **注意：** 
>1.  不支持文件系统、控制台。
>2.  不支持fstat，fsync，writev接口。

## 安全函数库<a name="section127921457151719"></a>

### 概述<a name="section5848918131818"></a>

危险函数依赖于程序员对参数进行检查或保证空间能足够容纳所产生的结果，函数本身不对这些情况进行判断，即使有问题也不会给出错误的指示。C11标准中对于过时的不安全的危险函数定义了对应的安全函数（\_s版本的函数），相比危险函数，安全函数对照C11标准进行了相应的安全增强，会对入参以及不同的错误情况进行判断，降低操作不当所引入的安全风险。下表列举了危险函数以及对应的安全函数，TA代码中涉及到相关危险函数的必须使用安全函数。

**表 10**  危险函数以及对应的安全函数

<a name="table18216829674"></a>
<table><thead align="left"><tr id="row1221612296715"><th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.1"><p id="p1280043317715"><a name="p1280043317715"></a><a name="p1280043317715"></a>危险函数</p>
</th>
<th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.2"><p id="p880019332710"><a name="p880019332710"></a><a name="p880019332710"></a>安全函数</p>
</th>
</tr>
</thead>
<tbody><tr id="row122166291776"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p198006331074"><a name="p198006331074"></a><a name="p198006331074"></a>memcpy</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p16800533473"><a name="p16800533473"></a><a name="p16800533473"></a>memcpy_s</p>
</td>
</tr>
<tr id="row421610291977"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p080012331073"><a name="p080012331073"></a><a name="p080012331073"></a>memmove</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p1680013331779"><a name="p1680013331779"></a><a name="p1680013331779"></a>memmove_s</p>
</td>
</tr>
<tr id="row152161229975"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p1480013336712"><a name="p1480013336712"></a><a name="p1480013336712"></a>memset</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p4800103314713"><a name="p4800103314713"></a><a name="p4800103314713"></a>memset_s</p>
</td>
</tr>
<tr id="row02163297719"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p1580017338710"><a name="p1580017338710"></a><a name="p1580017338710"></a>strcpy</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p20800233575"><a name="p20800233575"></a><a name="p20800233575"></a>strcpy_s</p>
</td>
</tr>
<tr id="row132161629777"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p680016331977"><a name="p680016331977"></a><a name="p680016331977"></a>strncpy</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p1880013335719"><a name="p1880013335719"></a><a name="p1880013335719"></a>strncpy_s</p>
</td>
</tr>
<tr id="row1821642918715"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p180010331175"><a name="p180010331175"></a><a name="p180010331175"></a>strcat</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p158003331671"><a name="p158003331671"></a><a name="p158003331671"></a>strcat_s</p>
</td>
</tr>
<tr id="row72167299712"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p780014331373"><a name="p780014331373"></a><a name="p780014331373"></a>strncat</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p188017331171"><a name="p188017331171"></a><a name="p188017331171"></a>strncat_s</p>
</td>
</tr>
<tr id="row17217202917714"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p780113331478"><a name="p780113331478"></a><a name="p780113331478"></a>strtok</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p128013333711"><a name="p128013333711"></a><a name="p128013333711"></a>strtok_s</p>
</td>
</tr>
<tr id="row1021717291676"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p980111331072"><a name="p980111331072"></a><a name="p980111331072"></a>snprintf</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p98011331277"><a name="p98011331277"></a><a name="p98011331277"></a>snprintf_s</p>
</td>
</tr>
<tr id="row92174291974"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p380114333720"><a name="p380114333720"></a><a name="p380114333720"></a>vsnprintf</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p20801183316710"><a name="p20801183316710"></a><a name="p20801183316710"></a>vsnprintf_s</p>
</td>
</tr>
</tbody>
</table>

详细描述请参考头文件<securec.h\>。

安全函数包含部分如下特性:

-   强化边界检查：在接口参数中增加一个buffer长度的参数，在长度参数正确情况下不会出现溢出。
-   保证结果字符串以’\\0’结尾，避免访问buffer边界之外的信息。
-   发现缓冲区溢出发生，将目的缓冲区的首字节置零。
-   增加错误返回值，便于程序员快速进行错误定位。
-   增强入参检查。
-   增加入参的内存重叠检查（memcpy\_sp宏中对于常量拷贝不进行内存重叠检查）。
-   定义SECUREC\_STRING\_MAX\_LEN和SECUREC\_MEM\_MAX\_LEN宏，可以通过其限定字符串和内存操作时的最大长度。
-   定义SECUREC\_ERROR\_INVALID\_PARAMTER、SECUREC\_ERROR\_INVALID\_RANGE和SECUREC\_ERROR\_BUFFER\_OVERLAP宏，其会在参数出错时进行统一处理，用户可自定义发生错误时的函数处理行为。

### 内存复制<a name="section1620304312183"></a>

memcpy\_s：

```
errno_t memcpy_s(void* dest, size_t destMax, const void* src, size_t count);
```

复制源缓冲区的数据到目的缓冲区。

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：1）多了一个参数：目的缓冲区总大小；  2）系统函数返回值为指针，安全函数返回值为整型。
>-   调用函数时，注意判断返回值是否成功，否则有可能操作结果和预期不一致。
>-   某些出错情况下，会对目的缓冲区清0，具体参考如上**表1**。
>-   进行字符串拷贝时，不会自动在末尾添加结束符。（对字符串操作，建议使用字符串操作函数）

-   示例

<figure>

```
 #include "securec.h" 
 #include <string.h> 
  
 #define BUFFER_SIZE 11 
  
 int main() 
 { 
     char str1[BUFFER_SIZE] = "0123456789"; 
     char str2[BUFFER_SIZE] = {0x00}; 
     errno_t rc =  EOK; 
  
     rc = memcpy_s(str2,  BUFFER_SIZE, str1, BUFFER_SIZE - 1); 
     printf("rc = %d,  %s\n", rc, str2); 
  
     /* count is bigger than destMax, return ERANGE_AND_RESET  and dest is reset. */ 
     rc = memcpy_s(str2,  BUFFER_SIZE, str1, BUFFER_SIZE + 1); 
     printf("rc = %d, %s\n", rc,  str2); 
  
     /* initialize */ 
     rc = memcpy_s(str2,  BUFFER_SIZE, str1, BUFFER_SIZE - 1); 
     printf("rc = %d, %s\n", rc,  str2); 
  
     /* overlap, return EOVERLAP_AND_RESET and dest is reset.  */ 
     rc = memcpy_s(str2,  BUFFER_SIZE, str2 + 2, BUFFER_SIZE - 1); 
     printf("rc = %d, %s\n", rc,  str2); 
  
     return 0; 
 }
运行结果：
rc =  0, 0123456789
rc =  162
rc =  0, 0123456789
rc =  182
```

</figure>

memmove\_s：

```
errno_t memmove_s(void* dest,size_t destMax, const void* src, size_t count);
```

移动源缓冲区的数据到目的缓冲区

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：
>    -   多了一个参数：目的缓冲区总大小；
>    -   系统函数返回值为指针，安全函数返回值为整型。
>-   调用函数时，注意判断返回值是否成功，否则有可能操作结果和预期不一致。
>-   某些出错情况下，会对目的缓冲区清0，具体参考如上**表2**。

-   示例

<figure>

```
 #include "securec.h" 
 #include <string.h> 
  
 #define BUFFER_SIZE 11 
  
 int main() 
 { 
     char str[BUFFER_SIZE] = "0123456789"; 
     errno_t rc =  EOK; 
  
     printf("Before:  %s\n", str); 
  
     /* Move six bytes from the start of the  string 
     * to a new position shifted  by one byte. To protect against 
     * buffer overrun, the  secure version of memmove requires the 
     * the length of the  destination string to be specified. 
     */ 
  
     rc = memmove_s(str,  BUFFER_SIZE, str + 1, 6); 
     printf("After: rc = %d, %s\n", rc,  str); 
  
     /* count is bigger than destMax, return ERANGE_AND_RESET  and dest is reset. */ 
     rc = memmove_s(str,  BUFFER_SIZE, str + 1, BUFFER_SIZE + 100); 
     printf("Later: rc = %d, %s\n",  rc, str); 
  
     return 0; 
 }
运行结果:
Before: 0123456789
After: rc = 0,  123456789
Later: rc =  162
```

</figure>

### 内存初始化<a name="section9113185921817"></a>

memset\_s：

```
errno_t memset_s(void* dest, size_t destMax, int c, size_t count);
```

复制源缓冲区的数据到目的缓冲区

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：1）多了一个参数：目的缓冲区总大小；  2）系统函数返回值为指针，安全函数返回值为整型。
>-   调用函数时，注意判断返回值是否成功，否则有可能操作结果和预期不一致。

-   示例

<figure>

```
 #include "securec.h" 
 #include <string.h> 
  
 #define BUFFER_SIZE 40 
  
 int main() 
 { 
     char buffer[BUFFER_SIZE] =  "This is a test of the memset  function"; 
     errno_t rc =  EOK; 
  
     printf( "Before: %s\n", buffer ); 
  
     rc = memset_s(buffer,  BUFFER_SIZE, '*', 20 ); 
     printf( "After: rc = %d, %s\n", rc, buffer  ); 
  
     /* count is bigger than destMax, return ERANGE_AND_RESET  and destMax size is set. */ 
     rc = memset_s(buffer,  BUFFER_SIZE, '*', BUFFER_SIZE + 1 ); 
     printf( "Later: rc = %d, %s\n", rc, buffer  ); 
  
     return  0; 
 }
 
 运行结果：
 Before: This is a test of the memset function 
 After: rc = 0, ********************e memset  function 
 Later: rc = 162,  ****************************************
```

</figure>

### 字符串复制<a name="section320518115198"></a>

strcpy\_s：

```
errno_t strcpy_s(char* strDest, size_t destMax, const char* strSrc);
```

复制源字符串到目的缓冲区

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：1）、多了一个参数：目的缓冲区总大小； 2）、系统函数返回值为指针，安全函数返回值为整型。
>-   调用函数时，注意判断返回值是否成功，否则有可能操作结果和预期不一致。
>-   某些出错情况下，会对目的缓冲区首字符置0，具体参考如上**表1**。
>源字符串必须含有结束符。

-   示例

<figure>

```
 #include "securec.h" 
 #include <stdio.h> 
  
 #define SMALL_BUF_SIZE 10 
 #define BIG_BUF_SIZE 100 
  
 int main() 
 { 
     char str1[BIG_BUF_SIZE] = {0x00}; 
     char str2[SMALL_BUF_SIZE] = {0x00}; 
     char *str3 = str1 +  4; 
     errno_t rc =  EOK; 
  
     rc = strcpy_s(str1,  BIG_BUF_SIZE, "Security Design  Department"); 
     printf("rc = %d,  %s\n", rc, str1); 
  
     /* strSrc length + 1 is bigger than destMax, return  ERANGE_AND_RESET and strDest is reset. */ 
     rc = strcpy_s(str2,  SMALL_BUF_SIZE, "Security Design"); 
     printf("rc = %d,  %s\n", rc, str2); 
  
     memset_s(str1,  BIG_BUF_SIZE, 0x41, BIG_BUF_SIZE); 
     /* overlap, return EOVERLAP_AND_RESET and strDest is reset.  */ 
     rc = strcpy_s(str1,  BIG_BUF_SIZE, str3); 
     printf("rc = %d,  %s\n", rc, str1); 
  
     return 0; 
 }

运行结果：
rc  = 0, Security Design Department
rc  = 162,
rc  = 182,
```

</figure>

strncpy\_s：

```
errno_t strncpy_s(char* strDest, size_t destMax, const char* strSrc, size_t count);
```

复制指定长度的源字符串到目的缓冲区。

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：1）多了一个参数：目的缓冲区总大小； 2）系统函数返回值为指针，安全函数返回值为整型。
>-   调用函数时，注意判断返回值是否成功，否则有可能操作结果和预期不一致。
>-   某些出错情况下，会对目的缓冲区首字符置0，具体参考如上**表2**。
>-   源字符串必须含有结束符。
>-   调用该函数时建议传入的参数destMax  大于 count以保证有截断功能。
>-   安全函数strncpy\_s执行完后保证strDest有’\\0’结尾,而危险函数strncpy不保证。
>-   当  count 大于 strlen\(strSrc\)  时，  strncpy函数会复制完字符串后在strDest中填充count-strlen\(strSrc\)个’\\0’字符，而strncpy\_s函数不做填充。

-   示例

<figure>

```
 #define SMALL_BUF_SIZE 10
 #define BIG_BUF_SIZE 100 
  
 int main() 
 { 
     char str1[BIG_BUF_SIZE] =  {0x00}; 
     char str2[BIG_BUF_SIZE] =  "security design  department"; 
     char *str3 = str1 +  4; 
     char str4[SMALL_BUF_SIZE] =  {0x00}; 
     errno_t rc =  EOK; 
  
     rc = strncpy_s(str1,  BIG_BUF_SIZE, str2, 15); 
     printf("rc = %d, %s\n", rc, str1); 
  
     /* count is bigger than destMax, return ERANGE_AND_RESET  and dest is reset. */ 
     rc = strncpy_s(str4,  SMALL_BUF_SIZE, "security Design Department", 15); 
     printf("rc = %d, %s\n", rc, str4); 
  
     memset_s(str1,  BIG_BUF_SIZE, 0x41, BIG_BUF_SIZE); 
     /* overlap, return EOVERLAP_AND_RESET and dest is reset.  */ 
     rc = strncpy_s(str1,  BIG_BUF_SIZE, str3, 15); 
     printf("rc = %d, %s\n", rc, str1); 
  
     return 0; 
 }

运行结果：
rc =  0, security design
rc =  162,
rc =  182,
```

</figure>

### 字符串连接<a name="section3307826101911"></a>

strcat\_s：

```
errno_t strcat_s(char* strDest, size_t destMax, const char* strSrc);
```

将源字符串连接到目的字符串后面。

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：
>    -   多了一个参数：目的缓冲区总大小；
>    -   系统函数返回值为指针，安全函数返回值为整型。
>-   调用函数时，注意判断返回值是否成功，否则有可能操作结果和预期不一致。
>-   某些出错情况下，会对目的缓冲区的首字符置0，具体参考如上**表1**。

-   示例

<figure>

```
 #include "securec.h" 
 #include <stdio.h> 
  
 #define BUFFER_SIZE  64 
  
 int main() 
 { 
     char   str1[BUFFER_SIZE] = {’’}; 
     char   str2[BUFFER_SIZE] = {’b’}; 
     errno_t rc = EOK; 
  
     rc = strcat_s(str1,  3, str2); 
     printf("rc = %d, %s\n", rc, str1); 
  
  
     /*strdest in destMax don’t have ‘\0’*/ 
     memset_s(str1, BUFFER_SIZE,'a',4); 
     memset_s(str2, BUFFER_SIZE,'b',4); 
     rc = strcat_s(str1, 4,  str2); 
     printf("rc = %d, %s\n", rc, str1); 
  
     /*destmax isn’t enough for strSrc*/ 
     memset_s(str1, BUFFER_SIZE,'a',4); 
     memset_s(str2, BUFFER_SIZE,'b',4); 
     rc = strcat_s(str1,8, str2); 
     printf("rc = %d, %s\n", rc, str1); 
  
     /*dest , src overlap*/ 
     rc = strncat_s(str2,8, str2+2,1); 
     printf("rc = %d, %s\n", rc, str2); 
  
     return 0; 
 }

运行结果:
rc = 0, ab
rc = 150,
rc = 162,
rc = 182,
```

</figure>

strncat\_s：

```
errno_t strncat_s(char* strDest, size_t destMax, const char* strSrc, size_t count);
```

将指定长度的源字符串连接到目的字符串后面。

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：
>    -   多了一个参数：目的缓冲区总大小；
>    -   系统函数返回值为指针，安全函数返回值为整型。
>-   调用函数时，注意判断返回值是否成功，否则有可能操作结果和预期不一致。
>-   某些出错情况下，会对目的缓冲区的首字符置0，具体参考如上**表2**。

-   示例

<figure>

```
 #include "securec.h" 
 #include <stdio.h> 
  
 #define BUFFER_SIZE  64 
  
 int main() 
 { 
     char   str1[BUFFER_SIZE] = {’’}; 
     char   str2[BUFFER_SIZE] = {’b’}; 
     errno_t rc = EOK; 
  
     rc = strncat_s(str1, 3, str2,  1); 
     printf("rc = %d, %s\n", rc , str1); 
  
     /*strdest in destMax don’t have  ‘\0’*/ 
     memset_s(str1, BUFFER_SIZE,’a’,4); 
     memset_s(str2, BUFFER_SIZE,’b’,4); 
     rc = strncat_s(str1, 4, str2,  2); 
     printf("rc  = %d, %s\n", rc ,  str1); 
  
     /*destmax isn’t enough for  strSrc*/ 
     memset_s(str1, BUFFER_SIZE,’a’,4); 
     memset_s(str2, BUFFER_SIZE,’b’,4); 
     rc = strncat_s(str1,8, str2,5); 
     printf("rc = %d, %s\n", rc , str1); 
  
     /*dest , src overlap*/ 
     rc = strncat_s(str2,8, str2+2,1); 
     printf("rc = %d, %s\n", rc , str2); 
  
     return 0; 
 }

运行结果:
rc  = 0, ab
rc  = 150,
rc  = 162,
rc  = 182,
```

</figure>

### 字符串分割<a name="section85731515152610"></a>

strtok\_s：

```
errno_t strtok_s(char* strToken, const char* strDelimit, char** context);
```

将字符串按照指定的分隔符分割成子字符串。

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：多了一个参数context：保存调用strtok\_s后的下个子字符串的首地址，当不再有子字符串时，context指向被分割字符串结尾。
>-   当在被分割字符串中没有找到分隔符时，如果被分割字符串长度大于0，会返回被分割字符串首地址，否则返回NULL。
>-   以逗号分隔符为例，顺序调用strtok\_s\(str,&p\), strtok\_s\(NULL,&p\),  strtok\_s\(NULL,&p\)函数的返回值与不同分割字符串的关系如下：

**表 11**  返回值与字符串关系

<a name="table18556455195912"></a>
<table><tbody><tr id="row7556165595917"><td class="cellrowborder" rowspan="2" valign="top"><p id="p156641710803"><a name="p156641710803"></a><a name="p156641710803"></a><strong id="b86641109012"><a name="b86641109012"></a><a name="b86641109012"></a><span>被分割字符串</span></strong></p>
</td>
<td class="cellrowborder" colspan="3" valign="top"><p id="p1766414101808"><a name="p1766414101808"></a><a name="p1766414101808"></a><strong id="b1866414101608"><a name="b1866414101608"></a><a name="b1866414101608"></a><span>返回值</span></strong></p>
</td>
</tr>
<tr id="row155765565917"><td class="cellrowborder" valign="top"><p id="p146654101808"><a name="p146654101808"></a><a name="p146654101808"></a>strtok_s(str,&amp;p)</p>
</td>
<td class="cellrowborder" valign="top"><p id="p15665610307"><a name="p15665610307"></a><a name="p15665610307"></a>strtok_s(NULL,&amp;p)</p>
</td>
<td class="cellrowborder" valign="top"><p id="p17665111019017"><a name="p17665111019017"></a><a name="p17665111019017"></a>strtok_s(NULL,&amp;p)</p>
</td>
</tr>
<tr id="row955745525914"><td class="cellrowborder" valign="top" width="25%"><p id="p19665171014011"><a name="p19665171014011"></a><a name="p19665171014011"></a>str = "aaa"</p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p46652010100"><a name="p46652010100"></a><a name="p46652010100"></a>"aaa"</p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p06652101907"><a name="p06652101907"></a><a name="p06652101907"></a>NULL</p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p466511105013"><a name="p466511105013"></a><a name="p466511105013"></a>_</p>
</td>
</tr>
<tr id="row75571655115910"><td class="cellrowborder" valign="top" width="25%"><p id="p1066512101208"><a name="p1066512101208"></a><a name="p1066512101208"></a><span>str =</span> <span>""</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p4665181017014"><a name="p4665181017014"></a><a name="p4665181017014"></a><span>NULL</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p166654101307"><a name="p166654101307"></a><a name="p166654101307"></a>_</p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p17665111018016"><a name="p17665111018016"></a><a name="p17665111018016"></a>_</p>
</td>
</tr>
<tr id="row4557145516596"><td class="cellrowborder" valign="top" width="25%"><p id="p176651510509"><a name="p176651510509"></a><a name="p176651510509"></a><span>str =</span> <span>","</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p766510101102"><a name="p766510101102"></a><a name="p766510101102"></a><span>NULL</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p0665610101"><a name="p0665610101"></a><a name="p0665610101"></a>_</p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p106657106015"><a name="p106657106015"></a><a name="p106657106015"></a>_</p>
</td>
</tr>
<tr id="row195571055105919"><td class="cellrowborder" valign="top" width="25%"><p id="p26651510407"><a name="p26651510407"></a><a name="p26651510407"></a><span>str =</span> <span>",,"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p9665510409"><a name="p9665510409"></a><a name="p9665510409"></a><span>NULL</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p206657101204"><a name="p206657101204"></a><a name="p206657101204"></a>_</p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p066541016017"><a name="p066541016017"></a><a name="p066541016017"></a>_</p>
</td>
</tr>
<tr id="row6558115516595"><td class="cellrowborder" valign="top" width="25%"><p id="p1866520102008"><a name="p1866520102008"></a><a name="p1866520102008"></a><span>str =</span> <span>",aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p96655102010"><a name="p96655102010"></a><a name="p96655102010"></a><span>"aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p06651101902"><a name="p06651101902"></a><a name="p06651101902"></a><span>NULL</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p18666121018017"><a name="p18666121018017"></a><a name="p18666121018017"></a>_</p>
</td>
</tr>
<tr id="row1955865511594"><td class="cellrowborder" valign="top" width="25%"><p id="p166671012013"><a name="p166671012013"></a><a name="p166671012013"></a><span>str =</span> <span>",,aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p1666611018017"><a name="p1666611018017"></a><a name="p1666611018017"></a><span>"aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p76665101802"><a name="p76665101802"></a><a name="p76665101802"></a><span>NULL</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p106666102015"><a name="p106666102015"></a><a name="p106666102015"></a>_</p>
</td>
</tr>
<tr id="row1955805514595"><td class="cellrowborder" valign="top" width="25%"><p id="p466618103014"><a name="p466618103014"></a><a name="p466618103014"></a><span>str =</span> <span>"aaa,"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p566615101006"><a name="p566615101006"></a><a name="p566615101006"></a><span>"aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p12666210206"><a name="p12666210206"></a><a name="p12666210206"></a><span>NULL</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p1566617101809"><a name="p1566617101809"></a><a name="p1566617101809"></a>_</p>
</td>
</tr>
<tr id="row755810558594"><td class="cellrowborder" valign="top" width="25%"><p id="p146661510301"><a name="p146661510301"></a><a name="p146661510301"></a><span>str =</span> <span>"aaa,,"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p1566641012015"><a name="p1566641012015"></a><a name="p1566641012015"></a><span>"aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p1966613107017"><a name="p1966613107017"></a><a name="p1966613107017"></a><span>NULL</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p1166611108010"><a name="p1166611108010"></a><a name="p1166611108010"></a>_</p>
</td>
</tr>
<tr id="row155915515598"><td class="cellrowborder" valign="top" width="25%"><p id="p1666611101603"><a name="p1666611101603"></a><a name="p1666611101603"></a><span>str =</span> <span>"aaa,bbb"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p106663101504"><a name="p106663101504"></a><a name="p106663101504"></a><span>"aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p13666610804"><a name="p13666610804"></a><a name="p13666610804"></a><span>"bbb"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p126663101003"><a name="p126663101003"></a><a name="p126663101003"></a><span>NULL</span></p>
</td>
</tr>
<tr id="row9559555135919"><td class="cellrowborder" valign="top" width="25%"><p id="p13666131015010"><a name="p13666131015010"></a><a name="p13666131015010"></a><span>str =</span> <span>"aaa,,bbb"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p176668102003"><a name="p176668102003"></a><a name="p176668102003"></a><span>"aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p196661108012"><a name="p196661108012"></a><a name="p196661108012"></a><span>"bbb"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p1366601016011"><a name="p1366601016011"></a><a name="p1366601016011"></a><span>NULL</span></p>
</td>
</tr>
<tr id="row1455915555596"><td class="cellrowborder" valign="top" width="25%"><p id="p8667510002"><a name="p8667510002"></a><a name="p8667510002"></a><span>str =</span> <span>"aaa,bbb,"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p196672010506"><a name="p196672010506"></a><a name="p196672010506"></a><span>"aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p6667201020015"><a name="p6667201020015"></a><a name="p6667201020015"></a><span>"bbb"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p13667161011011"><a name="p13667161011011"></a><a name="p13667161011011"></a><span>NULL</span></p>
</td>
</tr>
<tr id="row175591455145914"><td class="cellrowborder" valign="top" width="25%"><p id="p466713102019"><a name="p466713102019"></a><a name="p466713102019"></a><span>str =</span> <span>"aaa,bbb,,"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p1166751019014"><a name="p1166751019014"></a><a name="p1166751019014"></a><span>"aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p9667010504"><a name="p9667010504"></a><a name="p9667010504"></a><span>"bbb"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p16667910402"><a name="p16667910402"></a><a name="p16667910402"></a><span>NULL</span></p>
</td>
</tr>
</tbody>
</table>

-   示例

<figure>

```
 #include "securec.h" 
 #include <stdio.h> 
  
 int main() 
 { 
     char string1[64]  = "A string\tof ,,tokens\nand some  more tokens"; 
     char string2[64]  = "Another string\n\tparsed at the same  time."; 
     char seps[] =  " ,\t\n"; 
     char *token1 =  NULL; 
     char *token2 =  NULL; 
     char *next_token1  = NULL; 
     char *next_token2= NULL; 
  
     printf( "Tokens:\n" ); 
  
     /* Establish string and get the first token:  */ 
     token1 = strtok_s(string1, seps,  &next_token1); 
     token2 = strtok_s(string2, seps,  &next_token2); 
  
     /* While there are tokens in "string1" or "string2"  */ 
     while ((token1 !=  NULL) || (token2 != NULL)) 
     { 
         /* Get next token: */ 
         if (token1 !=  NULL) 
         { 
             printf("  %s\n", token1); 
             token1 = strtok_s(NULL,  seps, &next_token1); 
         } 
         if (token2 !=  NULL) 
         { 
             printf("        %s\n", token2); 
             token2 = strtok_s (NULL, seps,  &next_token2); 
         } 
     } 
  
     return 0; 
 }

运行结果:
Tokens:
more
same
some
the
and
at
tokens
parsed
of
string
string
Another
time.
tokens
```

</figure>

### 格式化输出<a name="section1696173315284"></a>

snprintf\_s：

```
int snprintf_s(char* strDest, size_t destMax, size_t count, const char* format, ...);
```

将数据按照指定长度格式化输出到目的缓冲区。

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：
>    -   多了一个参数：目的缓冲区总大小；
>    -   count参数与系统函数的用法存在差异，参见参数表。
>-   调用函数时，注意判断返回值是否成功，否则有可能操作结果和预期不一致。
>-   某些出错情况下，会对目的缓冲区首字符清0，具体参考如上**表1**。
>-   调用该函数时建议传入的参数destMax 大于 count以保证有截断功能。
>-   确保输入的源数据与目标缓冲区不存在重叠。
>-   输入源数据中若是字符串则必须含有结束符。
>输入源数据的类型、个数必须与格式化控制字符串（format）中的类型、个数保持一致。

-   示例

<figure>

```
 #include "securec.h" 
  
 void  snprintf_sample() 
 { 
     char buffer[200]={0}; 
     char     *string = "computer"; 
     wchar_t *wstring =  L"Unicode"; 
  
     /* format strings. */ 
     int iRet = snprintf_s( buffer, 200, 20,  "%s,%ls", string, wstring); 
     printf("iRet = %d, buffer = %s.\n", iRet,  buffer); 
  
     /* test edge. */ 
     printf("------------------\n"); 
     iRet = snprintf_s( buffer, 3, 3, "%s",  "123"); 
     printf("iRet =  %d, buffer = %s.\n", iRet, buffer); 
  
     /* test edge. */ 
     printf("------------------\n"); 
     iRet = snprintf_s( buffer, 3, 2, "%s",  "123"); 
     printf("iRet = %d, buffer = %s.\n", iRet,  buffer); 
  
  
     return; 
 } 
  
 int  main() 
 { 
     snprintf_sample (); 
     return 0; 
 }

运行结果：
iRet = 16, buffer  = computer,Unicode.
iRet = -1, buffer  = .

iRet = -1, buffer  = 12.
```

</figure>

vsnprintf\_s：

```
int vsnprintf_s(char* strDest, size_t destMax, size_t count, const char* format, va_list arglis);
```

将参数列表\(_va\_list_\)数据按照指定长度格式化输出到目的缓冲区。

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：
>    -   多了一个参数：目的缓冲区总大小；
>    -   count参数与系统函数的用法存在差异，参见参数表。
>-   调用函数时，注意判断返回值是否成功，否则有可能操作结果和预期不一致。
>-   某些出错情况下，会对目的缓冲区首字符清0，具体参考如上表2。
>-   调用该函数时建议传入的参数destMax 大于 count以保证有截断功能。
>-   确保输入的源数据与目标缓冲区不存在重叠。
>-   输入源数据中若是字符串则必须含有结束符。
>-   输入源数据的类型、个数必须与格式化控制字符串（format）中的类型、个数保持一致。

-   示例1

<figure>

```
 #include "securec.h" 
  
 void FormatOutput(char* formatstring, ...) 
 { 
     va_list args; 
     int nSize =  0; 
     char buff[10]; 
     memset_s(buff,sizeof(buff), 0, sizeof(buff)); 
     va_start(args,  formatstring); 
     nSize = vsnprintf_s( buff, sizeof(buff), 8,  formatstring, args); 
     printf("nSize: %d,  buff: %s\n", nSize, buff); 
 } 
  
 int main() 
 { 
     FormatOutput("%s  %s", "Hi", "there"); 
     FormatOutput("%s  %s", "Hi", "there!"); 
  
     return 0; 
 }

运行结果：
nSize: 8, buff: Hi there
nSize: -1, buff: Hi there
```

</figure>

-   示例2

<figure>

```
 void vsnprintf_base(char * format, ...)
 { 
     va_list  args; 
     char  buffer[100]; 
     int iRet =  -1; 
  
     va_start(args,  format); 
     iRet = vsnprintf_s(buffer,  100, 20, format, args); 
     printf("iRet = %d, buffer  = %s.\n", iRet, buffer); 
  
     return; 
 } 
 void vsnprintf_sample(void) 
 { 
     char    *string = "computer"; 
     wchar_t *wstring =  L"Unicode"; 
  
     /* format strings.  */ 
     vsnprintf_base("%s,%ls",  string, wstring); 
  
     /* test  edge.*/ 
     printf("------------------\n"); 
     vsnprintf_base("%s",  "12345678901234567890a"); 
  
     return; 
 } 
  
 int main() 
 { 
     vsnprintf_sample(); 
     return  0; 
 }

运行结果：
iRet = 16, buffer = computer,Unicode.
iRet = -1, buffer = 12345678901234567890.
```

</figure>

