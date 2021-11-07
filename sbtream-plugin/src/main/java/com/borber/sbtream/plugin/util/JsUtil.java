package com.borber.sbtream.plugin.util;

import org.jetbrains.annotations.NotNull;

import javax.script.Invocable;
import javax.script.ScriptEngine;
import javax.script.ScriptEngineManager;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.Reader;
import java.nio.charset.StandardCharsets;

/**
 * @ClassName FileUtil
 * @Description Js工具箱
 * @Author Borber
 * @Date 2021/6/27 16:37
 * @Version 0.0.1
 **/

public class JsUtil {


    /*
     * @Author Borber
     * @Description 运行指定路径下的 js 文件中的指定方法
     * @Date 16:49 2021/6/27
     * @Param [path, method]
     * @return java.lang.String
     **/

    public static String RunMethodByJsFile(@NotNull String path, @NotNull String method, Object args) throws IOException {
        ScriptEngineManager manager = new ScriptEngineManager();
        ScriptEngine engine = manager.getEngineByName("js");
        FileInputStream fileInputStream = new FileInputStream(path);

        try (Reader scriptReader = new InputStreamReader(fileInputStream, StandardCharsets.UTF_8)) {

            // 读js文件
            engine.eval(scriptReader);

            if (engine instanceof Invocable) {
                // 调用JS方法
                Invocable invocable = (Invocable) engine;
                return (String) invocable.invokeFunction(method,args);
            }
        } catch (Exception e) {
            if (e instanceof NoSuchMethodException){
                return "No such method";
            }else {
                return "Some thing wrong: \r\n" + e.getMessage();
            }
        }

        return "Some thing wrong check the RunJsFile";
    }
}

