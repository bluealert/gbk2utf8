将一个目录下的所有C++源文件从GBK编码转换为UTF8编码：

```
find . -name "*.h" -o -name "*.cpp" | xargs -n 1 gbk2utf8
```
