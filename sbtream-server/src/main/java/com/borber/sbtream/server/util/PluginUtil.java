package com.borber.sbtream.server.util;

import com.borber.sbtream.plugin.annotations.Plugin;
import com.borber.sbtream.plugin.api.Api;
import org.springframework.stereotype.Component;

import java.io.File;
import java.io.IOException;
import java.net.URL;
import java.net.URLClassLoader;
import java.util.ArrayList;
import java.util.Enumeration;
import java.util.Objects;
import java.util.jar.JarEntry;
import java.util.jar.JarFile;
import static com.borber.sbtream.server.constans.SBtreamConfigConstans.PLUGINS_PATH;

public class PluginUtil {

    public static ArrayList<Api> getPlugins() throws Exception {
        File pluginPath =  new File(PLUGINS_PATH);
        if (!pluginPath.exists()) {
            throw new IOException("Path does not exist");
        }
        if (!pluginPath.isDirectory()) {
            throw new IOException("The path is not a directory");
        }
        ArrayList<File> files = new ArrayList<>();
        ArrayList<Api> plugins = new ArrayList<>();

        for (File file : Objects.requireNonNull(pluginPath.listFiles())) {
            if (file.getName().endsWith(".jar")) {
                files.add(file);
            }
        }

        for (File f : files) {
            JarFile jar = new JarFile(f);
            Enumeration<JarEntry> entries = jar.entries();
            while (entries.hasMoreElements()) {
                JarEntry entry = entries.nextElement();
                if (entry.isDirectory() || !entry.getName().endsWith(".class")) {
                    continue;
                }
                String className = entry.getName().substring(0, entry.getName().length() - 6);
                className = className.replace('/', '.');
                Class<?> clazz = new URLClassLoader(new URL[]{f.toURI().toURL()}, Thread.currentThread().getContextClassLoader()).loadClass(className);
                if (clazz.getAnnotation(Plugin.class) != null) {
                    plugins.add((Api) clazz.newInstance());
                }
            }
        }
        System.out.println(plugins);
        return plugins;
    }
}
