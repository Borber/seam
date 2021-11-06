package com.borber.sbtream.server.util;

import java.io.*;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.List;

/**
 * @ClassName FileUtil
 * @Description TODO
 * @Author Borber
 * @Date 2021/7/15 下午5:31
 * @Version 0.0.1
 **/
public class FileUtil {

    /**
     * @Author Borber
     * @Description 获取 指定路径下 jar 文件 的数量
     * @Date 16:46 2021/6/27
     * @Param [path]
     * @return int
     **/
    public int getNumberOfJar(String path){
        List<String> fileList = getAllJar(path);
        if (fileList == null) {
            return 0;
        }
        return fileList.size();
    }

    /**
     * @Author Borber
     * @Description 获取指定路径下 所有的 jar 文件路径
     * @Date 16:47 2021/6/27
     * @Param [path]
     * @return java.util.List<java.lang.String>
     **/
    public List<String> getAllJar(String path){
        return getAllFileWithFix(path,"jar",true);
    }

    /**
     * @Author Borber
     * @Description 获取具有指定的前缀或后缀的文件
     * @Date 16:46 2021/6/27
     * @Param [path, fix, suffix]
     * @return java.util.List<java.lang.String>
     **/
    public static List<String> getAllFileWithFix(String path,String fix,boolean suffix){
        File[] tempList = getAllFiles(path);
        if (tempList == null) {
            return null;
        }
        List <String> fileList = new ArrayList<>();
        for (File f: tempList){
            String n = f.getPath();
            if (suffix && n.endsWith(fix)){
                fileList.add(f.getPath());
            }
            if (!suffix && n.startsWith(fix)){
                fileList.add(f.getPath());
            }
        }
        return fileList;
    }

    /**
     * @Author Borber
     * @Description 获取指定地址下的所有文件
     * @Date 16:45 2021/6/27
     * @Param [path]
     * @return java.io.File[]
     **/
    public static File[] getAllFiles(String path){
        File file = new File(path);
        return file.listFiles();
    }

    /**
     * @Author Borber
     * @Description 储存 json 为 文件
     * @Date  2021/6/27 20:14
     * @Param [path, json]
     **/
    public static void saveFile(String path, String data) throws IOException {
        File file = new File(path);
        FileOutputStream fop = new FileOutputStream(file);
        OutputStreamWriter writer = new OutputStreamWriter(fop, StandardCharsets.UTF_8);
        writer.write(data);
        writer.close();
        fop.close();
    }

    /**
     * @Author Borber
     * @Description 加载 json 文件
     * @Date  2021/6/27 20:15
     * @Param [path]
     * @return java.lang.String
     **/
    public static String loadFile(String path) throws IOException {
        File file = new File(path);
        FileInputStream fip = new FileInputStream(file);
        InputStreamReader reader = new InputStreamReader(fip);
        StringBuilder sb = new StringBuilder();
        while (reader.ready()){
            sb.append((char) reader.read());
        }
        reader.close();
        fip.close();
        return sb.toString();
    }
}
