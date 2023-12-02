package util

fun readFileUsingGetResource(fileName: String): String =
    ClassLoader.getSystemClassLoader().getResource(fileName).readText(Charsets.UTF_8)