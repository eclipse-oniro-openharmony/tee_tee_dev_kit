# TEE标准C库支持<a name="ZH-CN_TOPIC_0000001241900905"></a>

支持大多数的POSIX接口，具体支持情况请参考：POSIX:[https://mirror.math.princeton.edu/pub/oldlinux/download/c953.pdf](https://mirror.math.princeton.edu/pub/oldlinux/download/c953.pdf)

支持绝大多数的libc接口。目前使用的musl-1.2.0/libc库请参考下表。

>![](public_sys-resources/icon-note.gif) **说明：** 
>-   不支持文件系统、控制台。
>-   不支持fstat，fsync，writev接口。
>-   stdio中的printf函数目前不支持文件系统，文件操作只支持标准输入输出。

**表 1**  标准C支持列表

<a name="table7336617112614"></a>
<table><thead align="left"><tr id="row1633681714266"><th class="cellrowborder" valign="top" width="47.88%" id="mcps1.2.3.1.1"><p id="p1653154512717"><a name="p1653154512717"></a><a name="p1653154512717"></a>模块</p>
</th>
<th class="cellrowborder" valign="top" width="52.12%" id="mcps1.2.3.1.2"><p id="p1253114512274"><a name="p1253114512274"></a><a name="p1253114512274"></a>函数接口名</p>
</th>
</tr>
</thead>
<tbody><tr id="row6340121710262"><td class="cellrowborder" rowspan="42" valign="top" width="47.88%" headers="mcps1.2.3.1.1 "><p id="p18534845102711"><a name="p18534845102711"></a><a name="p18534845102711"></a>pthread</p>
</td>
<td class="cellrowborder" valign="top" width="52.12%" headers="mcps1.2.3.1.2 "><p id="p553444592710"><a name="p553444592710"></a><a name="p553444592710"></a>sem_getvalue</p>
</td>
</tr>
<tr id="row10341181782614"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p753424552719"><a name="p753424552719"></a><a name="p753424552719"></a>sem_init</p>
</td>
</tr>
<tr id="row534110172268"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p1853444542719"><a name="p1853444542719"></a><a name="p1853444542719"></a>sem_post</p>
</td>
</tr>
<tr id="row203411176269"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p1553474572712"><a name="p1553474572712"></a><a name="p1553474572712"></a>sem_wait</p>
</td>
</tr>
<tr id="row5341131742616"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p10534184519273"><a name="p10534184519273"></a><a name="p10534184519273"></a>pthread_mutex_destroy</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_mutex_init</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_mutex_lock</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_mutex_trylock</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_mutex_unlock</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_mutexattr_destroy</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_mutexattr_init</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_mutexattr_setprotocol</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_mutexattr_setpshared</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_mutexattr_settype</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_mutexattr_setrobust</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_spin_destroy</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_spin_init</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_spin_lock/p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_spin_trylock</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_spin_unlock</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_cond_broadcast</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_cond_destroy</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_cond_init</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_cond_signal</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_cond_wait</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_attr_destroy</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_attr_getstack</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_attr_getstacksize</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_attr_init</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_attr_setstack</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_attr_setstacksize</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_create</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_equal</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_exit</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_getspecific</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_join</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_key_create</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_key_delete</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_once</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_self</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_setschedprio</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>pthread_setspecific</p>
</td>
</tr>
<tr id="row6340121710262"><td class="cellrowborder" rowspan="6" valign="top" width="47.88%" headers="mcps1.2.3.1.1 "><p id="p18534845102711"><a name="p18534845102711"></a><a name="p18534845102711"></a>malloc</p>
</td>
<td class="cellrowborder" valign="top" width="52.12%" headers="mcps1.2.3.1.2 "><p id="p553444592710"><a name="p553444592710"></a><a name="p553444592710"></a>aligned_alloc</p>
</td>
</tr>
<tr id="row10341181782614"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p753424552719"><a name="p753424552719"></a><a name="p753424552719"></a>calloc</p>
</td>
</tr>
<tr id="row534110172268"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p1853444542719"><a name="p1853444542719"></a><a name="p1853444542719"></a>malloc</p>
</td>
</tr>
<tr id="row203411176269"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p1553474572712"><a name="p1553474572712"></a><a name="p1553474572712"></a>realloc</p>
</td>
</tr>
<tr id="row5341131742616"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p10534184519273"><a name="p10534184519273"></a><a name="p10534184519273"></a>free</p>
</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>posix_memalign</p>
</td>
</tr>
<tr id="row18341121732619"><td class="cellrowborder" rowspan="2" valign="top" width="47.88%" headers="mcps1.2.3.1.1 "><p id="p125341945182720"><a name="p125341945182720"></a><a name="p125341945182720"></a>mman</p>
</td>
<td class="cellrowborder" valign="top" width="52.12%" headers="mcps1.2.3.1.2 "><p id="p7534184532717"><a name="p7534184532717"></a><a name="p7534184532717"></a>mmap</p>
</td>
</tr>
<tr id="row534131772614"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12534184513273"><a name="p12534184513273"></a><a name="p12534184513273"></a>munmap</p>
</td>
</tr>
<tr id="row434101742618"><td class="cellrowborder" rowspan="3" valign="top" width="47.88%" headers="mcps1.2.3.1.1 "><p id="p253419457279"><a name="p253419457279"></a><a name="p253419457279"></a>time</p>
</td>
<td class="cellrowborder" valign="top" width="52.12%" headers="mcps1.2.3.1.2 "><p id="p16534124510274"><a name="p16534124510274"></a><a name="p16534124510274"></a>gettimeofday</p>
</td>
</tr>
<tr id="row53411917172619"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p11534124522712"><a name="p11534124522712"></a><a name="p11534124522712"></a>strftime</p>
</td>
</tr>
<tr id="row5341101712617"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p19535845152713"><a name="p19535845152713"></a><a name="p19535845152713"></a>time</p>
</td>
</tr>
<tr id="row1334131792612"><td class="cellrowborder" rowspan="6" valign="top" width="47.88%" headers="mcps1.2.3.1.1 "><p id="p105357458277"><a name="p105357458277"></a><a name="p105357458277"></a>stdio</p>
</td>
<td class="cellrowborder" valign="top" width="52.12%" headers="mcps1.2.3.1.2 "><p id="p75354454278"><a name="p75354454278"></a><a name="p75354454278"></a>printf</p>
</td>
</tr>
<tr id="row63421517192616"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p2535154592716"><a name="p2535154592716"></a><a name="p2535154592716"></a>scanf</p>
</td>
</tr>
<tr id="row83421817132613"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p105351045132712"><a name="p105351045132712"></a><a name="p105351045132712"></a>snprintf</p>
</td>
</tr>
<tr id="row434291782619"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12535545132715"><a name="p12535545132715"></a><a name="p12535545132715"></a>sprintf</p>
</td>
</tr>
<tr id="row19342517122616"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p3535245132711"><a name="p3535245132711"></a><a name="p3535245132711"></a>vsnprintf</p>
</td>
</tr>
<tr id="row1342161792613"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p353518454270"><a name="p353518454270"></a><a name="p353518454270"></a>vsprintf</p>
</td>
</tr>
<tr id="row734291715264"><td class="cellrowborder" rowspan="2" valign="top" width="47.88%" headers="mcps1.2.3.1.1 "><p id="p13535045172712"><a name="p13535045172712"></a><a name="p13535045172712"></a>errno</p>
</td>
<td class="cellrowborder" valign="top" width="52.12%" headers="mcps1.2.3.1.2 "><p id="p25351645132719"><a name="p25351645132719"></a><a name="p25351645132719"></a>errno</p>
</td>
</tr>
<tr id="row11342181711269"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p11535164515276"><a name="p11535164515276"></a><a name="p11535164515276"></a>strerror</p>
</td>
</tr>
<tr id="row5342151710264"><td class="cellrowborder" valign="top" width="47.88%" headers="mcps1.2.3.1.1 "><p id="p10535945152714"><a name="p10535945152714"></a><a name="p10535945152714"></a>exit</p>
</td>
<td class="cellrowborder" valign="top" width="52.12%" headers="mcps1.2.3.1.2 "><p id="p8535124519274"><a name="p8535124519274"></a><a name="p8535124519274"></a>abort</p>
</td>
</tr>
<tr id="row11342117182611"><td class="cellrowborder" valign="top" width="47.88%" headers="mcps1.2.3.1.1 "><p id="p85358458277"><a name="p85358458277"></a><a name="p85358458277"></a>unistd</p>
</td>
<td class="cellrowborder" valign="top" width="52.12%" headers="mcps1.2.3.1.2 "><p id="p1753517452273"><a name="p1753517452273"></a><a name="p1753517452273"></a>getpid</p>
</td>
</tr>
<tr id="row1034281772615"><td class="cellrowborder" rowspan="4" valign="top" width="47.88%" headers="mcps1.2.3.1.1 "><p id="p1553514592714"><a name="p1553514592714"></a><a name="p1553514592714"></a>locale</p>
</td>
<td class="cellrowborder" valign="top" width="52.12%" headers="mcps1.2.3.1.2 "><p id="p753574512278"><a name="p753574512278"></a><a name="p753574512278"></a>setlocale</p>
</td>
</tr>
<tr id="row334220171263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p115353451276"><a name="p115353451276"></a><a name="p115353451276"></a>strcoll</p>
</td>
</tr>
<tr id="row13343151762618"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p95351145142716"><a name="p95351145142716"></a><a name="p95351145142716"></a>strxfrm</p>
</td>
</tr>
<tr id="row173433172262"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p0535104512273"><a name="p0535104512273"></a><a name="p0535104512273"></a>strtod</p>
</td>
</tr>
<tr id="row2343131732614"><td class="cellrowborder" rowspan="3" valign="top" width="47.88%" headers="mcps1.2.3.1.1 "><p id="p95351945132716"><a name="p95351945132716"></a><a name="p95351945132716"></a>multibyte</p>
</td>
<td class="cellrowborder" valign="top" width="52.12%" headers="mcps1.2.3.1.2 "><p id="p25351345122715"><a name="p25351345122715"></a><a name="p25351345122715"></a>mbrtowc</p>
</td>
</tr>
<tr id="row1234321772614"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p185354453273"><a name="p185354453273"></a><a name="p185354453273"></a>wcrtomb</p>
</td>
</tr>
<tr id="row7343151762613"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p115361445112716"><a name="p115361445112716"></a><a name="p115361445112716"></a>wctob</p>
</td>
</tr>
<tr id="row1034351714262"><td class="cellrowborder" rowspan="4" valign="top" width="47.88%" headers="mcps1.2.3.1.1 "><p id="p135361645112710"><a name="p135361645112710"></a><a name="p135361645112710"></a>prng</p>
</td>
<td class="cellrowborder" valign="top" width="52.12%" headers="mcps1.2.3.1.2 "><p id="p19536134517272"><a name="p19536134517272"></a><a name="p19536134517272"></a>srandom</p>
</td>
</tr>
<tr id="row73431717102610"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p17536134518272"><a name="p17536134518272"></a><a name="p17536134518272"></a>initstate</p>
</td>
</tr>
<tr id="row17343181742615"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p2536144515271"><a name="p2536144515271"></a><a name="p2536144515271"></a>setstate</p>
</td>
</tr>
<tr id="row16343121711263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p185362459278"><a name="p185362459278"></a><a name="p185362459278"></a>random</p>
</td>
</tr>
<tr id="row63431217102612"><td class="cellrowborder" rowspan="17" valign="top" width="47.88%" headers="mcps1.2.3.1.1 "><p id="p1153618451278"><a name="p1153618451278"></a><a name="p1153618451278"></a>string</p>
</td>
<td class="cellrowborder" valign="top" width="52.12%" headers="mcps1.2.3.1.2 "><p id="p1553620456277"><a name="p1553620456277"></a><a name="p1553620456277"></a>memchr</p>
</td>
</tr>
<tr id="row93431217112613"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p14536154510275"><a name="p14536154510275"></a><a name="p14536154510275"></a>memcmp</p>
</td>
</tr>
<tr id="row17344417112619"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p153684515275"><a name="p153684515275"></a><a name="p153684515275"></a>memcpy</p>
</td>
</tr>
<tr id="row1934421712265"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p1653694592712"><a name="p1653694592712"></a><a name="p1653694592712"></a>memmove</p>
</td>
</tr>
<tr id="row934411712267"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p2536114510274"><a name="p2536114510274"></a><a name="p2536114510274"></a>memset</p>
</td>
</tr>
<tr id="row163441117142619"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p105361345112715"><a name="p105361345112715"></a><a name="p105361345112715"></a>strchr</p>
</td>
</tr>
<tr id="row1434451715262"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p175366451279"><a name="p175366451279"></a><a name="p175366451279"></a>strcmp</p>
</td>
</tr>
<tr id="row534471762616"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p353694514273"><a name="p353694514273"></a><a name="p353694514273"></a>strcpy</p>
</td>
</tr>
<tr id="row334461752611"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p1753694515274"><a name="p1753694515274"></a><a name="p1753694515274"></a>strlen</p>
</td>
</tr>
<tr id="row834419172265"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p0536745102720"><a name="p0536745102720"></a><a name="p0536745102720"></a>strncmp</p>
</td>
</tr>
<tr id="row1734415175268"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p55361345192717"><a name="p55361345192717"></a><a name="p55361345192717"></a>strncpy</p>
</td>
</tr>
<tr id="row183445174264"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p115365457273"><a name="p115365457273"></a><a name="p115365457273"></a>strnlen</p>
</td>
</tr>
<tr id="row1534401712611"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p19537245112712"><a name="p19537245112712"></a><a name="p19537245112712"></a>strrchr</p>
</td>
</tr>
<tr id="row13451617142620"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p125371456272"><a name="p125371456272"></a><a name="p125371456272"></a>strstr</p>
</td>
</tr>
<tr id="row15345181762614"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p95371845142718"><a name="p95371845142718"></a><a name="p95371845142718"></a>wcschr</p>
</td>
</tr>
<tr id="row10345151718267"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p0537124512279"><a name="p0537124512279"></a><a name="p0537124512279"></a>wcslen</p>
</td>
</tr>
<tr id="row2345111710263"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p145371645142718"><a name="p145371645142718"></a><a name="p145371645142718"></a>wmemchr</p>
</td>
</tr>
<tr id="row12345131715263"><td class="cellrowborder" rowspan="13" valign="top" width="47.88%" headers="mcps1.2.3.1.1 "><p id="p753714502717"><a name="p753714502717"></a><a name="p753714502717"></a>ctype</p>
</td>
<td class="cellrowborder" valign="top" width="52.12%" headers="mcps1.2.3.1.2 "><p id="p5537145142720"><a name="p5537145142720"></a><a name="p5537145142720"></a>isalpha</p>
</td>
</tr>
<tr id="row1434581792614"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p0537184511274"><a name="p0537184511274"></a><a name="p0537184511274"></a>isascii</p>
</td>
</tr>
<tr id="row5345417182620"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p8537114513278"><a name="p8537114513278"></a><a name="p8537114513278"></a>isdigit</p>
</td>
</tr>
<tr id="row8345117122618"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p8537204582712"><a name="p8537204582712"></a><a name="p8537204582712"></a>islower</p>
</td>
</tr>
<tr id="row034551782617"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p253716455273"><a name="p253716455273"></a><a name="p253716455273"></a>isprint</p>
</td>
</tr>
<tr id="row4345101713267"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p1053713455279"><a name="p1053713455279"></a><a name="p1053713455279"></a>isspace</p>
</td>
</tr>
<tr id="row16345201752616"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p853764562716"><a name="p853764562716"></a><a name="p853764562716"></a>iswctype</p>
</td>
</tr>
<tr id="row33455179264"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p18537114522715"><a name="p18537114522715"></a><a name="p18537114522715"></a>iswdigit</p>
</td>
</tr>
<tr id="row83461317192614"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p16537154512710"><a name="p16537154512710"></a><a name="p16537154512710"></a>iswlower</p>
</td>
</tr>
<tr id="row93461117152620"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p353713458276"><a name="p353713458276"></a><a name="p353713458276"></a>iswspace</p>
</td>
</tr>
<tr id="row19346161711268"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p75371145202717"><a name="p75371145202717"></a><a name="p75371145202717"></a>iswupper</p>
</td>
</tr>
<tr id="row123461517162613"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p3537245152710"><a name="p3537245152710"></a><a name="p3537245152710"></a>towupper</p>
</td>
</tr>
<tr id="row1734671714268"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p175381245172715"><a name="p175381245172715"></a><a name="p175381245172715"></a>towlower</p>
</td>
</tr>
<tr id="row6346117112615"><td class="cellrowborder" rowspan="16" valign="top" width="47.88%" headers="mcps1.2.3.1.1 "><p id="p2538174512714"><a name="p2538174512714"></a><a name="p2538174512714"></a>math</p>
</td>
<td class="cellrowborder" valign="top" width="52.12%" headers="mcps1.2.3.1.2 "><p id="p19538184572712"><a name="p19538184572712"></a><a name="p19538184572712"></a>atan</p>
</td>
</tr>
<tr id="row8346131782613"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p15381945172717"><a name="p15381945172717"></a><a name="p15381945172717"></a>ceil</p>
</td>
</tr>
<tr id="row734617176261"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p953844582716"><a name="p953844582716"></a><a name="p953844582716"></a>ceilf</p>
</td>
</tr>
<tr id="row134610174264"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p053824511271"><a name="p053824511271"></a><a name="p053824511271"></a>copysignl</p>
</td>
</tr>
<tr id="row13346917152610"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p13538104522717"><a name="p13538104522717"></a><a name="p13538104522717"></a>exp</p>
</td>
</tr>
<tr id="row334661712269"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p5538154512710"><a name="p5538154512710"></a><a name="p5538154512710"></a>fabs</p>
</td>
</tr>
<tr id="row2347181722616"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p2538184522716"><a name="p2538184522716"></a><a name="p2538184522716"></a>floor</p>
</td>
</tr>
<tr id="row123471517122616"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p1453874562717"><a name="p1453874562717"></a><a name="p1453874562717"></a>frexp</p>
</td>
</tr>
<tr id="row153471917112616"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p14538134512271"><a name="p14538134512271"></a><a name="p14538134512271"></a>frexpl</p>
</td>
</tr>
<tr id="row234771717261"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p10538245102714"><a name="p10538245102714"></a><a name="p10538245102714"></a>log</p>
</td>
</tr>
<tr id="row1334711179265"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p185381245142719"><a name="p185381245142719"></a><a name="p185381245142719"></a>log2</p>
</td>
</tr>
<tr id="row103471317132612"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p20538144512275"><a name="p20538144512275"></a><a name="p20538144512275"></a>pow</p>
</td>
</tr>
<tr id="row133472178262"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p35389459277"><a name="p35389459277"></a><a name="p35389459277"></a>roundf</p>
</td>
</tr>
<tr id="row234731713261"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p4538154522715"><a name="p4538154522715"></a><a name="p4538154522715"></a>scalbn</p>
</td>
</tr>
<tr id="row1347917182619"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p85381845192718"><a name="p85381845192718"></a><a name="p85381845192718"></a>scalbnl</p>
</td>
</tr>
<tr id="row1134717178267"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p11538845102718"><a name="p11538845102718"></a><a name="p11538845102718"></a>sqrt</p>
</td>
</tr>
<tr id="row13348101715261"><td class="cellrowborder" rowspan="14" align="left" valign="top" width="47.88%" headers="mcps1.2.3.1.1 "><p id="p195391445102713"><a name="p195391445102713"></a><a name="p195391445102713"></a>stdlib</p>
</td>
<td class="cellrowborder" valign="top" width="52.12%" headers="mcps1.2.3.1.2 "><p id="p1153954552713"><a name="p1153954552713"></a><a name="p1153954552713"></a>abs</p>
</td>
</tr>
<tr id="row4348101732615"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p1853910450272"><a name="p1853910450272"></a><a name="p1853910450272"></a>atof</p>
</td>
</tr>
<tr id="row12348111782617"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p12539124562717"><a name="p12539124562717"></a><a name="p12539124562717"></a>atoi</p>
</td>
</tr>
<tr id="row123482178261"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p95391945102713"><a name="p95391945102713"></a><a name="p95391945102713"></a>atol</p>
</td>
</tr>
<tr id="row163481117142610"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p35391745172718"><a name="p35391745172718"></a><a name="p35391745172718"></a>atoll</p>
</td>
</tr>
<tr id="row6348217132613"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p4539164511276"><a name="p4539164511276"></a><a name="p4539164511276"></a>bsearch</p>
</td>
</tr>
<tr id="row12348191714269"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p195391245102717"><a name="p195391245102717"></a><a name="p195391245102717"></a>div</p>
</td>
</tr>
<tr id="row1634810170260"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p10539124513274"><a name="p10539124513274"></a><a name="p10539124513274"></a>ecvt</p>
</td>
</tr>
<tr id="row103481517182617"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p75392457277"><a name="p75392457277"></a><a name="p75392457277"></a>imaxabs</p>
</td>
</tr>
<tr id="row1834817175260"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p854014552719"><a name="p854014552719"></a><a name="p854014552719"></a>llabs</p>
</td>
</tr>
<tr id="row1349121792612"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p1154054562718"><a name="p1154054562718"></a><a name="p1154054562718"></a>qsort</p>
</td>
</tr>
<tr id="row15349117192614"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p14540645172719"><a name="p14540645172719"></a><a name="p14540645172719"></a>strtoul</p>
</td>
</tr>
<tr id="row134910176267"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p135401445172712"><a name="p135401445172712"></a><a name="p135401445172712"></a>strtol</p>
</td>
</tr>
<tr id="row183494173268"><td class="cellrowborder" valign="top" headers="mcps1.2.3.1.1 "><p id="p3540114519272"><a name="p3540114519272"></a><a name="p3540114519272"></a>wcstod</p>
</td>
</tr>
</tbody>
</table>

