# TEE TA接口指南<a name="ZH-CN_TOPIC_0000001196701012"></a>

-   [3.1 TA的API接口](#section1827174413371)
-   [3.2 TA入口函数](#section1736115211372)
-   [3.3 安全存储](#section1265118373811)
-   [3.3.1 安全存储概述](#section1147512643814)
-   [使用场景](#section1443234517117)
-   [使用注意事项](#section73695311130)
-   [相关接口](#section19433151766)
-   [3.3.2 安全存储开发指导](#section8463163517388)
-   [3.3.2.1 存储密钥](#section127865873819)
-   [场景说明](#section143491044197)
-   [实例步骤](#section54172661916)
-   [实例代码](#section20236154071919)
-   [3.3.2.2 存储文件](#section413714417396)
-   [场景说明](#section121024517124)
-   [实例步骤](#section1272017448221)
-   [实例代码](#section1312146192312)
-   [3.3.2.3 使用enumerator](#section798251363920)
-   [场景说明](#section88380272417)
-   [实例步骤](#section352861519248)
-   [实例代码](#section13978630152415)
-   [3.3.2.4 常见问题](#section135971912394)
-   [打开文件失败，返回错误码ffff0008](#section44551991222)
-   [打开文件失败，返回错误码ffff0003](#section124828202212)
-   [3.4加解密](#section1646110252391)
-   [概述](#section19905102422715)
-   [加解密支持算法](#section7885121218201)
-   [密钥长度要求](#section158246431176)
-   [加解密常见问题](#section53591633153611)
    -   [密钥长度单位](#section17753135455011)
    -   [AE算法接口参数单位](#section146312014535)

-   [3.5 安全时间](#section63711528193920)
-   [3.6 TA异常](#section197719315396)

## 3.1 TA的API接口<a name="section1827174413371"></a>

TA的API接口在SDK中include/TA目录下。TA头文件在TEE支持情况如下：

<a name="table1974162111333"></a>
<table><thead align="left"><tr id="row1397412123310"><th class="cellrowborder" valign="top" width="100%" id="mcps1.1.2.1.1"><p id="p15475205614337"><a name="p15475205614337"></a><a name="p15475205614337"></a>头文件</p>
</th>
</tr>
</thead>
<tbody><tr id="row3974102114338"><td class="cellrowborder" valign="top" width="100%" headers="mcps1.1.2.1.1 "><p id="p1999891715252"><a name="p1999891715252"></a><a name="p1999891715252"></a>tee_arith_api.h</p>
<p id="p1199851772517"><a name="p1199851772517"></a><a name="p1199851772517"></a>tee_core_api.h</p>
<p id="p14998121711254"><a name="p14998121711254"></a><a name="p14998121711254"></a>tee_crypto_api.h</p>
<p id="p299819175253"><a name="p299819175253"></a><a name="p299819175253"></a>tee_defines.h</p>
<p id="p1099871718251"><a name="p1099871718251"></a><a name="p1099871718251"></a>tee_mem_mgmt_api.h</p>
<p id="p099881713254"><a name="p099881713254"></a><a name="p099881713254"></a>tee_mem_monitoring_api.h</p>
<p id="p139983172258"><a name="p139983172258"></a><a name="p139983172258"></a>tee_object_api.h</p>
<p id="p4998181713259"><a name="p4998181713259"></a><a name="p4998181713259"></a>tee_property_api.h</p>
<p id="p13998617142518"><a name="p13998617142518"></a><a name="p13998617142518"></a>tee_time_api.h</p>
<p id="p39981617112519"><a name="p39981617112519"></a><a name="p39981617112519"></a>tee_trusted_storage_api.h</p>
</td>
</tr>
<tr id="row14975102143315"><td class="cellrowborder" valign="top" width="100%" headers="mcps1.1.2.1.1 "><p id="p159981617112517"><a name="p159981617112517"></a><a name="p159981617112517"></a>ext/oemkey.h</p>
<p id="p199821716253"><a name="p199821716253"></a><a name="p199821716253"></a>ext/tee_crypto_hal.h</p>
<p id="p799851742517"><a name="p799851742517"></a><a name="p799851742517"></a>ext/tee_drv_client.h</p>
<p id="p13998181716259"><a name="p13998181716259"></a><a name="p13998181716259"></a>ext/tee_ext_api.h</p>
<p id="p14998141720257"><a name="p14998141720257"></a><a name="p14998141720257"></a>ext/tee_hw_ext_api.h</p>
<p id="p1199821720255"><a name="p1199821720255"></a><a name="p1199821720255"></a>ext/tee_log.h</p>
</td>
</tr>
</tbody>
</table>

## 3.2 TA入口函数<a name="section1736115211372"></a>

下表为TA入口函数。

入口函数必须按照表格中的函数来定义，否则TEE里加载TA二进制文件会有异常。

<a name="table381033124514"></a>
<table><thead align="left"><tr id="row11810339456"><th class="cellrowborder" valign="top" width="37.169999999999995%" id="mcps1.1.3.1.1"><p id="p1218517916453"><a name="p1218517916453"></a><a name="p1218517916453"></a>TA入口函数</p>
</th>
<th class="cellrowborder" valign="top" width="62.83%" id="mcps1.1.3.1.2"><p id="p3185294452"><a name="p3185294452"></a><a name="p3185294452"></a>函数描述</p>
</th>
</tr>
</thead>
<tbody><tr id="row188111131453"><td class="cellrowborder" valign="top" width="37.169999999999995%" headers="mcps1.1.3.1.1 "><p id="p101851392457"><a name="p101851392457"></a><a name="p101851392457"></a>TA_CreateEntryPoint</p>
</td>
<td class="cellrowborder" valign="top" width="62.83%" headers="mcps1.1.3.1.2 "><p id="p418515934514"><a name="p418515934514"></a><a name="p418515934514"></a>TA实例的构造函数，每个TA实例的生命周期中只被调用一次。</p>
</td>
</tr>
<tr id="row181103134518"><td class="cellrowborder" valign="top" width="37.169999999999995%" headers="mcps1.1.3.1.1 "><p id="p171869994517"><a name="p171869994517"></a><a name="p171869994517"></a>TA_DestroyEntryPoint</p>
</td>
<td class="cellrowborder" valign="top" width="62.83%" headers="mcps1.1.3.1.2 "><p id="p1418611934510"><a name="p1418611934510"></a><a name="p1418611934510"></a>TA实例的析构函数，TEE系统在销毁TA实例时调用此函数。</p>
</td>
</tr>
<tr id="row581163114517"><td class="cellrowborder" valign="top" width="37.169999999999995%" headers="mcps1.1.3.1.1 "><p id="p0186169124512"><a name="p0186169124512"></a><a name="p0186169124512"></a>TA_OpenSessionEntryPoint</p>
</td>
<td class="cellrowborder" valign="top" width="62.83%" headers="mcps1.1.3.1.2 "><p id="p171861298451"><a name="p171861298451"></a><a name="p171861298451"></a>在CA请求创建一个与TA的会话时，TEE系统会调用此函数来创建一个会话。</p>
</td>
</tr>
<tr id="row68111318451"><td class="cellrowborder" valign="top" width="37.169999999999995%" headers="mcps1.1.3.1.1 "><p id="p618614994517"><a name="p618614994517"></a><a name="p618614994517"></a>TA_CloseSessionEntryPoint</p>
</td>
<td class="cellrowborder" valign="top" width="62.83%" headers="mcps1.1.3.1.2 "><p id="p11186999459"><a name="p11186999459"></a><a name="p11186999459"></a>在CA发起关闭与TA的会话连接时，TEE系统会调用此函数。</p>
</td>
</tr>
<tr id="row28111835456"><td class="cellrowborder" valign="top" width="37.169999999999995%" headers="mcps1.1.3.1.1 "><p id="p71865984511"><a name="p71865984511"></a><a name="p71865984511"></a>TA_InvokeCommandEntryPoint</p>
</td>
<td class="cellrowborder" valign="top" width="62.83%" headers="mcps1.1.3.1.2 "><p id="p91861944519"><a name="p91861944519"></a><a name="p91861944519"></a>在CA发送指令给TA时，需要指定之前创建成功的会话，TEE系统收到请求后会调用此函数。</p>
</td>
</tr>
</tbody>
</table>

## 3.3 安全存储<a name="section1265118373811"></a>

## 3.3.1 安全存储概述<a name="section1147512643814"></a>

安全存储提供对数据的可信存储，并保证数据机密性、完整性、原子性、隔离性和不可复制性。

-   机密性：存储的数据为密文，保证数据不被泄露。
-   完整性：存储的数据有完整性保护，保证数据不被篡改。
-   原子性：保证数据的一致性。
-   隔离性：TA间存储的数据互相隔离，不能互相访问。
-   不可复制性：数据与设备进行绑定，复制到其它设备后无法访问。

## 使用场景<a name="section1443234517117"></a>

安全存储可用于存储需要加密保存的关键信息，例如指纹，银行卡信息等；或是存储TA中产生的数据，或是CA传递给TA的数据。

对于利用安全存储保存的数据，都需要开发具体的TA才能实现数据的保存。对于数据的来源视方案设计而定，可以是由REE的CA传入给TA，再由TA调用安全存储接口保存；也可以是TA内部生成再保存。对于需要在产线上写入的数据，同样也需要有TA配合单板软件和装备工具一起实现相应功能。

## 使用注意事项<a name="section73695311130"></a>

1.  文件名objectID长度限制为：128 Byte，路径名需要以“sec\_storage\_data/”开头，否则文件会被存储到非预期的位置。
2.  文件路径中可以存在相对路径“./” 但不能存在“../”，否则文件会创建失败。
3.  enumerator功能默认不打开。若要打开，则需要在manifest文件中增加字段gpd.ta.objectEnumEnable: true，重新签名TA，然后调用enumerator相关接口即可实现该功能。调用方式见[使用enumerator实例](#3323-使用enumerator)。
4.  TA存储的单个文件单次读写不能大于4M。
5.  安全存储使用写共享权限覆盖创建一个已经存在的文件时，更新文件之后，需要关闭文件再进行读取，否则可能出现读取的内容不是最新写入的内容。

## 相关接口<a name="section19433151766"></a>

安全存储相关接口以及内存申请相关接口的详细描述请参考tee\_trusted\_storage\_api.h、tee\_object\_api.h和tee\_mem\_mgmt\_api.h。

## 3.3.2 安全存储开发指导<a name="section8463163517388"></a>

## 3.3.2.1 存储密钥<a name="section127865873819"></a>

## 场景说明<a name="section143491044197"></a>

参考此开发指导，可完成以下功能场景开发：

-   为特定密钥申请内存空间。
-   使用随机数生成函数来生成密钥信息对称加解密。
-   创建永久Object用于存储密钥。

## 实例步骤<a name="section54172661916"></a>

1.  为AES密钥分配存储空间，AES密钥最长长度为32字节。
2.  将密钥信息存入临时内存空间中，用于密钥存取的验证。
3.  创建永久Object，将密钥存入。
4.  打开存储密钥的Object。
5.  关闭并删除存储AES密钥的Object。

## 实例代码<a name="section20236154071919"></a>

>![](public_sys-resources/icon-note.gif) **说明：** 
>存储密钥的类型仅支持GlobalPlatform所列类型，不支持的密钥可做为普通数据存储（存储方法见[存储文件实例](#3322-存储文件)）。

```
 TEE_Result store_key_sample(void)
{
    uint32_t storageID = TEE_OBJECT_STORAGE_PRIVATE;
    TEE_ObjectHandle transient_key = NULL;
    TEE_ObjectHandle persistent_key = NULL;
    TEE_Result ret;
    uint32_t w_flags = TEE_DATA_FLAG_ACCESS_WRITE;
    uint32_t r_flags = TEE_DATA_FLAG_ACCESS_READ;
    char attr_buffer[ARRAY_SIZE] = "test save attribute";
    void *objectID = "sec_storage_data/store_key_sample.txt";
    void *aes_key = NULL;

    /* Allocating Storage Space for Keys */
    ret = TEE_AllocateTransientObject(TEE_TYPE_AES, MAX_AES_KEY_SIZE, (&transient_key));
    if (ret != TEE_SUCCESS) {
        tloge("Failed to execute allocate transient object:ret = 0x%x", ret);
        return ret;
    }

    TEE_Attribute *aes_attribute = (TEE_Attribute *)TEE_Malloc(sizeof(TEE_Attribute), 0);
    if (aes_attribute == NULL) {
        tloge("Failed to malloc");
        TEE_FreeTransientObject(transient_key);
        return TEE_ERROR_OUT_OF_MEMORY;
    }

    TEE_InitRefAttribute(aes_attribute, TEE_ATTR_SECRET_VALUE, attr_buffer, sizeof(attr_buffer));
    ret = TEE_PopulateTransientObject(transient_key, aes_attribute, 1);
    if (ret != TEE_SUCCESS) {
        tloge("Failed to execute populate transient object:ret = %d", ret);
        goto cleanup_1;
    }

    /* Create a persistent file to store the key */
    ret = TEE_CreatePersistentObject(storageID, objectID, strlen(objectID), w_flags, transient_key, NULL, 0,
        (&persistent_key));
    if (ret != TEE_SUCCESS) {
        tloge("Failed to create object:ret = 0x%x", ret);
        goto cleanup_1;
    }
    TEE_CloseObject(persistent_key);

    /* Open the object to obtain key. */
    ret = TEE_OpenPersistentObject(storageID, objectID, strlen(objectID),
        r_flags | TEE_DATA_FLAG_ACCESS_WRITE_META, (&persistent_key));
    if (ret != TEE_SUCCESS) {
        tloge("Failed to execute open persistent object:ret = %x", ret);
        TEE_Free(aes_key);
        return ret;
    }
    TEE_CloseObject(persistent_key);
    persistent_key = NULL;
cleanup_1:
    TEE_Free(aes_attribute);
    TEE_FreeTransientObject(transient_key);
    return ret;
}
```

## 3.3.2.2 存储文件<a name="section413714417396"></a>

## 场景说明<a name="section121024517124"></a>

参考此开发指导，可完成以下功能场景开发：

-   创建永久Object，为永久Object分配内存空间。
-   将数据存入永久Object中。
-   打开永久Object，将写入的数据读出，判断是否写入成功。

## 实例步骤<a name="section1272017448221"></a>

1.  创建永久Object，申请内存空间，将数据写入永久Object。
2.  打开已经创建的文件，为打开的永久Object分配内存空间。
3.  读取存储的数据，并与之前写入的进行对比。
4.  如果相同，则证明写入成功。
5.  关闭已存储的数据文件。

## 实例代码<a name="section1312146192312"></a>

>![](public_sys-resources/icon-note.gif) **说明：** 
>安全存储的Object与文件操作类似，打开、创建和关闭必须成对操作，否则产生内存泄露。

```
TEE_Result store_data_sample(void) 
 { 
     uint32_t storageID = TEE_OBJECT_STORAGE_PRIVATE; 
     uint32_t r_flags = TEE_DATA_FLAG_ACCESS_READ; 
     uint32_t w_flags = TEE_DATA_FLAG_ACCESS_WRITE; 
     void *create_objectID = "sec_storage_data/store_data_sample.txt"; 
     TEE_ObjectHandle persistent_data = NULL; 
     TEE_ObjectInfo objectInfo = {0};
     TEE_Result ret; 
     char *write_buffer = "It is a test for persist object!"; 
     char *read_buffer = NULL; 
     uint32_t count = 0; 
  
     tlogd("store_data_sample start:============\n"); 
    /*
     * Create a permanent object: The flag is a write operation.
     * (The initialData, objectinfo, and attributes are written to the physical media.)
     */
     ret = TEE_CreatePersistentObject(storageID, create_objectID, strlen(create_objectID), w_flags, 
     TEE_HANDLE_NULL, NULL, 0, (&persistent_data)); 
     if (ret != TEE_SUCCESS) { 
         tloge("Failed to create file: ret = 0x%x\n", ret); 
         return ret; 
     } 
  
     ret = TEE_WriteObjectData(persistent_data, write_buffer, strlen(write_buffer)); 
     if (ret != TEE_SUCCESS) { 
         tloge("Failed to write file: ret = 0x%x\n", ret); 
         /*
          * After a file is opened or created, the exception branch must close the file.
          * Otherwise, memory leakage occurs.
          */
         TEE_CloseObject(persistent_data); 
         return ret; 
     } 
  
     TEE_CloseObject(persistent_data); 

     /* Open the created file and read data. */
     ret = TEE_OpenPersistentObject(storageID, create_objectID, strlen(create_objectID), 
     r_flags, (&persistent_data)); 
     if (ret != TEE_SUCCESS) { 
         tloge("Failed to open file:ret = 0x%x\n", ret); 
         return ret; 
     } 
  
     ret = TEE_GetObjectInfo1(persistent_data, &objectInfo);
     if (ret != TEE_SUCCESS) { 
         tloge("Failed to get info, ret = 0x%x\n", ret); 
         TEE_CloseObject(persistent_data); 
         return ret; 
     } 
  
     read_buffer = TEE_Malloc(objectInfo.dataSize + 1, 0); 
     if (read_buffer == NULL) {
         TEE_CloseObject(persistent_data); 
         return TEE_ERROR_OUT_OF_MEMORY; 
     }
 
     ret = TEE_ReadObjectData(persistent_data, read_buffer, objectInfo.dataSize, &count); 
     if (ret != TEE_SUCCESS) { 
         TEE_CloseObject(persistent_data); 
         TEE_Free(read_buffer); 
         return ret; 
     } 
  
     if (TEE_MemCompare(write_buffer, read_buffer, strlen(write_buffer)) != 0) { 
         TEE_CloseObject(persistent_data); 
         TEE_Free(read_buffer); 
         return TEE_FAIL; 
     } 
  
     TEE_CloseObject(persistent_data); 
     TEE_Free(read_buffer); 
     return TEE_SUCCESS; 
 }
```

## 3.3.2.3 使用enumerator<a name="section798251363920"></a>

## 场景说明<a name="section88380272417"></a>

参考此开发指导，可完成以下功能场景开发：

-   TA可以使用enumerator获取自己在private分区写入的文件信息

## 实例步骤<a name="section352861519248"></a>

1.  申请一个object enumerator handle。
2.  初始化object enumerator，开始列举给定存储空间的所有object。
3.  获取object enumerator中的下一个object。
4.  释放已分配资源的object enumerator handle。

## 实例代码<a name="section13978630152415"></a>

```
#define HASH_LEN 32
#define DIR_LEN 64
#define HASH_NAME_BUFF_LEN (2 * HASH_LEN + 1 + DIR_LEN)
TEE_Result enumerator_sample()
{
    TEE_Result ret;
    TEE_ObjectEnumHandle object_enumerator = NULL;
    TEE_ObjectInfo obj_info = {0};
    uint8_t objectId[HASH_NAME_BUFF_LEN] = {0};
    size_t object_id_len = HASH_NAME_BUFF_LEN;

    ret = TEE_AllocatePersistentObjectEnumerator(&object_enumerator);
    if (ret != TEE_SUCCESS) {
        tloge("Failed to allocate objectEnumerator");
        return ret;
    }

    ret = TEE_StartPersistentObjectEnumerator(object_enumerator, TEE_OBJECT_STORAGE_PRIVATE);
    if (ret != TEE_SUCCESS) {
        tloge("Failed to start object_enumerator");
        goto clean;
    }

    ret = TEE_GetNextPersistentObject(object_enumerator, &obj_info, objectId, &object_id_len);
    if (ret != TEE_SUCCESS) {
        tloge("Failed get next persistent object");
    }
clean:
    TEE_FreePersistentObjectEnumerator(object_enumerator);
    return ret;
}
```

## 3.3.2.4 常见问题<a name="section135971912394"></a>

## 打开文件失败，返回错误码ffff0008<a name="section44551991222"></a>

**问题现象**

打开文件，TEE日志中出现0xffff0008错误码打印，提示文件找不到。可能日志有：

```
cpu5/02083: 01/15 20:20:08.583 [task_ssa] [error] 2490:origin file open error ffff0008 and backup file open error ffff0008
cpu5/02083: 01/15 20:20:08.584 [task_ssa] [error] 2490:origin file open error ffff0008 and backup file open error ffff0008
cpu5/02083: 01/15 20:20:08.584 [task_ssa] [error] 1660:open fail ffff0008
```

**问题原因**

文件不存在。

**解决方法**

-   检查文件存储的分区和传入的路径是不是一致。
-   检查写入时和读取时传入的路径及文件名是否一致。

## 打开文件失败，返回错误码ffff0003<a name="section124828202212"></a>

**问题现象**

安全存储操作失败，TEE日志出现错误码0xffff0003打印

**问题原因**

目标文件已经被其他session打开，或者之前打开了文件未关闭。

**解决方法**

-   如果是多session并发访问同一个文件，需要在打开文件时增加共享读写的标记。
-   如果是打开的文件未关闭则需要关闭文件。

## 3.4加解密<a name="section1646110252391"></a>

## 概述<a name="section19905102422715"></a>

根据GP规范提供了常用的加解密、签名验签、摘要、消息认证码、密钥协商密码学算法及大数接口的实现。

加解密接口详细描述请参考头文件<tee\_crypto\_hal.h\>和<tee\_crypto\_api.h\>。

## 加解密支持算法<a name="section7885121218201"></a>

TEE支持的加解密算法如下表所示。

<a name="table189834229143"></a>
<table><thead align="left"><tr id="row598492261418"><th class="cellrowborder" valign="top" width="31.759999999999998%" id="mcps1.1.3.1.1"><p id="p15824182821419"><a name="p15824182821419"></a><a name="p15824182821419"></a>算法类别</p>
</th>
<th class="cellrowborder" valign="top" width="68.24%" id="mcps1.1.3.1.2"><p id="p79841922181410"><a name="p79841922181410"></a><a name="p79841922181410"></a>算法</p>
</th>
</tr>
</thead>
<tbody><tr id="row298402211141"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 "><p id="p109841322151410"><a name="p109841322151410"></a><a name="p109841322151410"></a>对称加密算法</p>
</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p143341651151416"><a name="p143341651151416"></a><a name="p143341651151416"></a>TEE_ALG_AES_ECB_NOPAD</p>
</td>
</tr>
<tr id="row139843225147"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p23340513140"><a name="p23340513140"></a><a name="p23340513140"></a>TEE_ALG_AES_CBC_NOPAD</p>
</td>
</tr>
<tr id="row698411227145"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p733475151417"><a name="p733475151417"></a><a name="p733475151417"></a>TEE_ALG_AES_ECB_PKCS5</p>
</td>
</tr>
<tr id="row169841522151410"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p113341251131411"><a name="p113341251131411"></a><a name="p113341251131411"></a>TEE_ALG_AES_CBC_PKCS5</p>
</td>
</tr>
<tr id="row398413225145"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p153341751131416"><a name="p153341751131416"></a><a name="p153341751131416"></a>TEE_ALG_AES_CTR</p>
</td>
</tr>
<tr id="row159841022151413"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p1333445191418"><a name="p1333445191418"></a><a name="p1333445191418"></a>TEE_ALG_AES_XTS</p>
</td>
</tr>
<tr id="row8984422131412"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p933425131411"><a name="p933425131411"></a><a name="p933425131411"></a>TEE_ALG_SM4_ECB_NOPAD</p>
</td>
</tr>
<tr id="row8262042101419"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p16334251141411"><a name="p16334251141411"></a><a name="p16334251141411"></a>TEE_ALG_SM4_CBC_NOPAD</p>
</td>
</tr>
<tr id="row441124261417"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p53340518141"><a name="p53340518141"></a><a name="p53340518141"></a>TEE_ALG_SM4_CBC_PKCS7</p>
</td>
</tr>
<tr id="row0567542181412"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p2334751111417"><a name="p2334751111417"></a><a name="p2334751111417"></a>TEE_ALG_SM4_CFB128</p>
</td>
</tr>
<tr id="row137291642141411"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p15334251101416"><a name="p15334251101416"></a><a name="p15334251101416"></a>TEE_ALG_SM4_CTR</p>
</td>
</tr>
<tr id="row685654251413"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 "><p id="p185674215140"><a name="p185674215140"></a><a name="p185674215140"></a>AE算法</p>
</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p19229181473"><a name="p19229181473"></a><a name="p19229181473"></a>TEE_ALG_SM4_GCM</p>
</td>
</tr>
<tr id="row181710430140"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p218418404170"><a name="p218418404170"></a><a name="p218418404170"></a>TEE_ALG_AES_CCM</p>
</td>
</tr>
<tr id="row14161174361417"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p118871304227"><a name="p118871304227"></a><a name="p118871304227"></a>TEE_ALG_AES_GCM</p>
</td>
</tr>
<tr id="row9317443121412"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 "><p id="p202205570246"><a name="p202205570246"></a><a name="p202205570246"></a>摘要算法</p>
</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p221314382516"><a name="p221314382516"></a><a name="p221314382516"></a>TEE_ALG_SHA256</p>
</td>
</tr>
<tr id="row946774341417"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p1521353132519"><a name="p1521353132519"></a><a name="p1521353132519"></a>TEE_ALG_SHA384</p>
</td>
</tr>
<tr id="row1462314316149"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p121317312517"><a name="p121317312517"></a><a name="p121317312517"></a>TEE_ALG_SHA512</p>
</td>
</tr>
<tr id="row679174313146"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p202131392516"><a name="p202131392516"></a><a name="p202131392516"></a>TEE_ALG_SM3</p>
</td>
</tr>
<tr id="row0121144151416"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 "><p id="p18121114411140"><a name="p18121114411140"></a><a name="p18121114411140"></a>MAC算法</p>
</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p1333951911254"><a name="p1333951911254"></a><a name="p1333951911254"></a>TEE_ALG_AES_CBC_MAC_NOPAD</p>
</td>
</tr>
<tr id="row13277104441413"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p1733981922518"><a name="p1733981922518"></a><a name="p1733981922518"></a>TEE_ALG_AES_CMAC</p>
</td>
</tr>
<tr id="row82161112152510"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p533971912510"><a name="p533971912510"></a><a name="p533971912510"></a>TEE_ALG_HMAC_SHA1</p>
</td>
</tr>
<tr id="row03661012142520"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p1634013191259"><a name="p1634013191259"></a><a name="p1634013191259"></a>TEE_ALG_HMAC_SHA256</p>
</td>
</tr>
<tr id="row1951531292516"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p14340111982515"><a name="p14340111982515"></a><a name="p14340111982515"></a>TEE_ALG_HMAC_SHA384</p>
</td>
</tr>
<tr id="row1766211220256"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p1534091918255"><a name="p1534091918255"></a><a name="p1534091918255"></a>TEE_ALG_HMAC_SHA512</p>
</td>
</tr>
<tr id="row178033129257"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p1734011918258"><a name="p1734011918258"></a><a name="p1734011918258"></a>TEE_ALG_HMAC_SM3</p>
</td>
</tr>
<tr id="row16121332512"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 "><p id="p140102512515"><a name="p140102512515"></a><a name="p140102512515"></a>非对称加密算法</p>
</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p8842103222510"><a name="p8842103222510"></a><a name="p8842103222510"></a>TEE_ALG_RSAES_PKCS1_V1_5</p>
</td>
</tr>
<tr id="row31575138257"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p10842193212253"><a name="p10842193212253"></a><a name="p10842193212253"></a>TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA256</p>
</td>
</tr>
<tr id="row6388182662513"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p184303219259"><a name="p184303219259"></a><a name="p184303219259"></a>TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA384</p>
</td>
</tr>
<tr id="row3545152611257"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p18843173213251"><a name="p18843173213251"></a><a name="p18843173213251"></a>TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA512</p>
</td>
</tr>
<tr id="row1701132682511"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p18843153222515"><a name="p18843153222515"></a><a name="p18843153222515"></a>TEE_ALG_RSA_NOPAD</p>
</td>
</tr>
<tr id="row1796332642518"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p158431932112511"><a name="p158431932112511"></a><a name="p158431932112511"></a>TEE_ALG_SM2_PKE</p>
</td>
</tr>
<tr id="row126362713258"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 "><p id="p1654914032519"><a name="p1654914032519"></a><a name="p1654914032519"></a>非对称签名算法</p>
</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p59191050192518"><a name="p59191050192518"></a><a name="p59191050192518"></a>TEE_ALG_RSASSA_PKCS1_V1_5_SHA256</p>
</td>
</tr>
<tr id="row15263163952513"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p11919195014258"><a name="p11919195014258"></a><a name="p11919195014258"></a>TEE_ALG_RSASSA_PKCS1_V1_5_SHA384</p>
</td>
</tr>
<tr id="row1141923911258"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p991905062512"><a name="p991905062512"></a><a name="p991905062512"></a>TEE_ALG_RSASSA_PKCS1_V1_5_SHA512</p>
</td>
</tr>
<tr id="row1569839182515"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p491975082519"><a name="p491975082519"></a><a name="p491975082519"></a>TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA256</p>
</td>
</tr>
<tr id="row473993919252"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p691955032511"><a name="p691955032511"></a><a name="p691955032511"></a>TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA384</p>
</td>
</tr>
<tr id="row643941316256"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p79196506251"><a name="p79196506251"></a><a name="p79196506251"></a>TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA512</p>
</td>
</tr>
<tr id="row17433164414147"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p5920950182513"><a name="p5920950182513"></a><a name="p5920950182513"></a>TEE_ALG_ECDSA_SHA256</p>
</td>
</tr>
<tr id="row1670155422514"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p19209502254"><a name="p19209502254"></a><a name="p19209502254"></a>TEE_ALG_ECDSA_SHA384</p>
</td>
</tr>
<tr id="row2857165417258"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p1920850122517"><a name="p1920850122517"></a><a name="p1920850122517"></a>TEE_ALG_ECDSA_SHA512</p>
</td>
</tr>
<tr id="row1743195572518"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p1692017504257"><a name="p1692017504257"></a><a name="p1692017504257"></a>TEE_ALG_ED25519</p>
</td>
</tr>
<tr id="row22175558256"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p14920105012251"><a name="p14920105012251"></a><a name="p14920105012251"></a>TEE_ALG_SM2_DSA_SM3</p>
</td>
</tr>
<tr id="row8673195517256"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 "><p id="p548061511263"><a name="p548061511263"></a><a name="p548061511263"></a>密钥协商算法</p>
</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p611262572611"><a name="p611262572611"></a><a name="p611262572611"></a>TEE_ALG_DH_DERIVE_SHARED_SECRET</p>
</td>
</tr>
<tr id="row1499121614264"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p61121025182611"><a name="p61121025182611"></a><a name="p61121025182611"></a>TEE_ALG_ECDH_DERIVE_SHARED_SECRET</p>
</td>
</tr>
<tr id="row567461619267"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p61122025142613"><a name="p61122025142613"></a><a name="p61122025142613"></a>TEE_ALG_X25519</p>
</td>
</tr>
<tr id="row1936211718261"><td class="cellrowborder" valign="top" width="31.759999999999998%" headers="mcps1.1.3.1.1 ">&nbsp;&nbsp;</td>
<td class="cellrowborder" valign="top" width="68.24%" headers="mcps1.1.3.1.2 "><p id="p31121251260"><a name="p31121251260"></a><a name="p31121251260"></a>TEE_TYPE_PBKDF2_HMAC</p>
</td>
</tr>
</tbody>
</table>

## 密钥长度要求<a name="section158246431176"></a>

-   RSA \>= 2048 bits
-   ECDSA \>= 224 bits
-   ECDH \>= 224 bits
-   DH  L \>= 2048 bits  N \>= 224 bits

## 加解密常见问题<a name="section53591633153611"></a>

### 密钥长度单位<a name="section17753135455011"></a>

TEE\_AllocateOperation、TEE\_AllocateTransientObject、TEE\_GenerateKey的密钥长度单位均为bit。

### AE算法接口参数单位<a name="section146312014535"></a>

TEE\_AEInit接口参数中，tagLen的单位为bit，nonceLen、AADLen、payloadLen的单位为byte。

## 3.5 安全时间<a name="section63711528193920"></a>

独立timer，准确的系统时钟，保证时钟准确、安全可靠，防止假冒。时钟只能在TEE里面设置、访问。

安全时间接口的详细描述请参考头文件<tee\_time\_api.h\>。

**表 1**  安全时间接口

<a name="table1807112065216"></a>
<table><thead align="left"><tr id="row280732075214"><th class="cellrowborder" valign="top" width="23.11%" id="mcps1.2.3.1.1"><p id="p380710205522"><a name="p380710205522"></a><a name="p380710205522"></a>函数接口</p>
</th>
<th class="cellrowborder" valign="top" width="76.89%" id="mcps1.2.3.1.2"><p id="p1880889185315"><a name="p1880889185315"></a><a name="p1880889185315"></a>使用须知</p>
</th>
</tr>
</thead>
<tbody><tr id="row1080722017523"><td class="cellrowborder" valign="top" width="23.11%" headers="mcps1.2.3.1.1 "><p id="p1280792010521"><a name="p1280792010521"></a><a name="p1280792010521"></a>TEE_GetSystemTime</p>
</td>
<td class="cellrowborder" valign="top" width="76.89%" headers="mcps1.2.3.1.2 "><p id="p2080815985320"><a name="p2080815985320"></a><a name="p2080815985320"></a>TEE系统时间从启动开始计时。</p>
</td>
</tr>
<tr id="row1280782035218"><td class="cellrowborder" valign="top" width="23.11%" headers="mcps1.2.3.1.1 "><p id="p48077203522"><a name="p48077203522"></a><a name="p48077203522"></a>TEE_Wait</p>
</td>
<td class="cellrowborder" valign="top" width="76.89%" headers="mcps1.2.3.1.2 "><a name="ol219995616554"></a><a name="ol219995616554"></a><ol id="ol219995616554"><li>TA创建的子线程或agent服务不支持使用TEE_Wait。</li><li>在TA2TA的调用方式中，第二级TA不支持使用TEE_Wait。</li><li>TEE_Wait不支持被取消，不支持永久等待。</li></ol>
</td>
</tr>
</tbody>
</table>

## 3.6 TA异常<a name="section197719315396"></a>

常见的TA异常包括两类，TA运行崩溃或者TA运行卡死。

-   TA运行崩溃：TA任意一个线程崩溃会触发整个TA进程被回收，即假如TA有多个session同时运行，其中一个session崩溃会导致多个session同时被回收。
-   TA运行卡死：TA运行卡死的情况下，TEE不会主动杀掉TA进程，只有CA同时被杀掉的情况下，TEE在一段时间的延时之后才会杀掉TA进程，假如TA有多个session，会同时被回收。

TA异常时返回给CA的错误码：

-   运行中的session如果被回收，TEE会向CA返回TEE\_ERROR\_TARGET\_DEAD错误码。
-   如果session在非运行状态时TA被回收，CA再次访问该session，由于TA被回收，TEE会返回TEE\_ERROR\_SERVICE\_NOT\_EXIST 错误码。

