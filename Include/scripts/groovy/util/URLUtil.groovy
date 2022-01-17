package util

class URLUtil {
    static def escapeURL(url) {
        return URLEncoder.encode(String.valueOf(url), "UTF-8")
    }
}