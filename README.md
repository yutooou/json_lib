

目前支持的序列化的类型有：

基础类型：
- int
- byte
- char
- short
- long
- boolean
- float
- double

包装类型
- Byte
- Short
- Integer
- Long
- Float
- Double
- Boolean
- Character
- String

集合类型
- java/util/Map及其子类型
- java/util/List及其子类型

自定义类
若需要对自定义类型进行序列化，则需要在改类型中完成该方法：
```java
public HashMap fieldTag() {
        HashMap<String, String> fieldTag = new HashMap<String, String>();
        Field[] fields = this.getClass().getDeclaredFields();
        for (int i = 0; i < fields.length; i++) {
            fieldTag.put(fields[i].getName(), fields[i].getType().getName());
        }
        return fieldTag;
    }
```
该方法可以将该类型的字段信息返回（ps：读结构这部分暂时没找到更好的办法）


```java
public class JSON {
    public static native String toJson(Object input);
    static {
        System.loadLibrary("json_lib");
    }
}
```
通过调用JSON类下的native方法toJson来完成序列化，当然要先使用`System.loadLibrary`来加载外部库



来到lib.rs，只有一个`pub extern "system" fn Java_JSON_toJson(env: JNIEnv, class: JClass, input: JObject) -> jstring`外部函数接口.
其函数名遵循`Java_包名_类名_自定义函数名_签名`的形式，参数的前两项`env: JNIEnv, class: JClass`决定了这是一个静态方法。最后的参数作为静态方法的参数`input: JObject`,其接受一个Object。

函数内即为序列化内容的实现，序列化实现分为两部分
一是`ObjectType::from_jvalue`来将传入的数据区分类型，生成实现trait `Convert`的结构体。直接instance of判断其类型
第二步是根据生成的结构体调用`serialize`来序列化，不同的类型serialize()的实现方式不同。

例如基础类型int、byte等，Jni已经帮助我们封装好了，直接做类型转化然后转为相应字符串即可（注意一些小细节例如float类型值为0时候）

而包装类型Integer、Byte等，就需要使用Interger等类中的方法来获取其值，例如Integer中的intValue `env.call_method(self.value, "intValue", "()I", &vec![])?.i()?;`

像自定义的类型，就需要通过桥梁jni来读取`fieldTag`来获取字段信息，在读取他们的字段值``env.get_field(self.value, name, tag)``后序列化，注意这里如果自定义类型中包含自定义类型的字段则会有递归的效果。

所有除了基础类型之外的类型本质上都是通过jni这个桥梁来判断类型并通过对象的方法来获取值。






对于这样的类：
```java
import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Person {

    public int t1;
    public byte t2;
    public char t3;
    public short t4;
    public long t5;
    public boolean t6;
    public float t7;
    public double t8;
    public Byte t9;
    public Short t10;
    public Integer t11;
    public Long t12;
    public Float t13;
    public Double t14;
    public Boolean t15;
    public Character t16;
    public String t17;
    public Phone phone;

    public List<String> list;

    public Map<String, Phone> map;

    public HashMap fieldTag() {
        HashMap<String, String> fieldTag = new HashMap<String, String>();
        Field[] fields = this.getClass().getDeclaredFields();
        for (int i = 0; i < fields.length; i++) {
            fieldTag.put(fields[i].getName(), fields[i].getType().getName());
        }
        return fieldTag;
    }

    public Person() {
        t6 = true;
        t9 = 3;
        t10 = 11;
        t11 = 572857;
        t12 = 247852778258L;
        t13 = 3.14F;
        t14 = 0.0;
        t15 = false;
        t3 = '张';
        t17 = "this is a string";
        phone = new Phone();
        list = new ArrayList<>();
        list.add("1");
        list.add("2");
        list.add("3");
        map = new HashMap<>();
        map.put("111", new Phone());
        map.put("222", new Phone());
    }

}

```
```json
import java.lang.reflect.Field;
import java.util.HashMap;

public class Phone {
    private String name;

    public Phone() {
        this.name = "菠萝手机";
    }
    public HashMap fieldTag() {
        HashMap<String, String> fieldTag = new HashMap<String, String>();
        Field[] fields = this.getClass().getDeclaredFields();
        for (int i = 0; i < fields.length; i++) {
            fieldTag.put(fields[i].getName(), fields[i].getType().getName());
        }
        return fieldTag;
    }
}
```


其序列化后的结果为
```json
{"t5": 0, "t13": 3.14, "list": ["1", "2", "3"], "t14": 0.0, "t16": null, "t10": 11, "map": {"111": {"name": "菠萝手机"}, "222": {"name": "菠萝手机"}}, "t9": 3, "t3": "张", "t4": 0, "t7": 0.0, "phone": {"name": "菠萝手机"}, "t11": 572857, "t6": true, "t15": false, "t12": 247852778258, "t17": "this is a string", "t8": 0.0, "t1": 0, "t2": 0}
```


