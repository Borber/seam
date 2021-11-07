package com.borber.sbtream.plugin.util;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import okhttp3.Call;
import okhttp3.OkHttpClient;
import okhttp3.Request;
import okhttp3.Response;

import java.io.IOException;
import java.util.Objects;

/**
 * @ClassName RequestUtil
 * @Description 用于实现网络请求的工具类
 * @Author Borber
 * @Date 2021/6/27 22:35
 * @Version 0.0.1
 **/

public class RequestUtil {
    private static final OkHttpClient client = new OkHttpClient();

    /**
     * @Author Borber
     * @Description OkHttp 样例请求
     * @Date  2021/6/27 23:16
     * @Param [url]
     * @return java.lang.String
     **/
    public static JsonNode sendPost(Request request) {
        Call call = client.newCall(request);
        Response response;
        String json;
        JsonNode jsonNode = null;
        ObjectMapper objectMapper = new ObjectMapper();
        try {
            response = call.execute();
            json = Objects.requireNonNull(response.body()).string();
            jsonNode = objectMapper.readTree(json);
        } catch (IOException e) {
            e.printStackTrace();
        }
        return jsonNode;
    }
}
