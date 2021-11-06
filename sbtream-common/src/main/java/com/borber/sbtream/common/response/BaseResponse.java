package com.borber.sbtream.common.response;

import java.io.Serializable;

/**
 * @ClassName BaseResponse
 * @Description 标准返回体
 * @Author Borber
 * @Date 2021/7/15 上午11:10
 * @Version 0.0.1
 **/
public class BaseResponse<T,V> implements Serializable {
    private static final long serialVersionUID = 4941396138988457180L;
    private Boolean success;
    private String code;
    private String message;
    private T data;
    private V extend;
    private String debugMessage;
    private String debugTrace;
    private String requestId;

    public BaseResponse() {
    }

    public BaseResponse(Boolean success, String code, String message) {
        this.success = success;
        this.code = code;
        this.message = message;
    }

    public BaseResponse(Boolean success, String code, String message, T data) {
        this.success = success;
        this.code = code;
        this.message = message;
        this.data = data;
    }

    public BaseResponse(Boolean success, String code, String message, T data, V extend, String debugMessage) {
        this.success = success;
        this.code = code;
        this.message = message;
        this.data = data;
        this.extend = extend;
        this.debugMessage = debugMessage;
    }

    public BaseResponse(Boolean success, String code, String message, T data, V extend) {
        this.success = success;
        this.code = code;
        this.message = message;
        this.data = data;
        this.extend = extend;
    }

    public static <T, V> BaseResponse<T, V> success(T data) {
        return new BaseResponse(true, "0", (String)null, data, (Object)null, "");
    }

    public static <T, V> BaseResponse<T, V> success(T data, V extend) {
        return new BaseResponse(true, "0", (String)null, data, extend);
    }

    public static <T, V> BaseResponse<T, V> success(String code, T data) {
        return new BaseResponse(true, code, (String)null, data);
    }

    public static <T, V> BaseResponse<T, V> success(String code, T data, V extend) {
        return new BaseResponse(true, code, (String)null, data, extend);
    }

    public static <T, V> BaseResponse<T, V> success(String code, String message, T data) {
        return new BaseResponse(true, code, message, data);
    }

    public static <T, V> BaseResponse<T, V> success(String code, String message, T data, V extend) {
        return new BaseResponse(true, code, message, data, extend);
    }

    public static <T, V> BaseResponse<T, V> fail(String code, String message) {
        return new BaseResponse(false, code, message, (Object)null);
    }

    public static <T, V> BaseResponse<T, V> fail(String code, String message, T data, V extend) {
        return new BaseResponse(false, code, message, data, extend);
    }

    public static <T, V> BaseResponse<T, V> fail(String code, String message, String debugMessage) {
        return new BaseResponse(false, code, message, (Object)null, (Object)null, debugMessage);
    }

    public Boolean getSuccess() {
        return this.success;
    }

    public String getCode() {
        return this.code;
    }

    public String getMessage() {
        return this.message;
    }

    public T getData() {
        return this.data;
    }

    public V getExtend() {
        return this.extend;
    }

    public String getDebugMessage() {
        return this.debugMessage;
    }

    public String getDebugTrace() {
        return this.debugTrace;
    }

    public String getRequestId() {
        return this.requestId;
    }

    public void setSuccess(final Boolean success) {
        this.success = success;
    }

    public void setCode(final String code) {
        this.code = code;
    }

    public void setMessage(final String message) {
        this.message = message;
    }

    public void setData(final T data) {
        this.data = data;
    }

    public void setExtend(final V extend) {
        this.extend = extend;
    }

    public void setDebugMessage(final String debugMessage) {
        this.debugMessage = debugMessage;
    }

    public void setDebugTrace(final String debugTrace) {
        this.debugTrace = debugTrace;
    }

    public void setRequestId(final String requestId) {
        this.requestId = requestId;
    }

    @Override
    public boolean equals(final Object o) {
        if (o == this) {
            return true;
        } else if (!(o instanceof BaseResponse)) {
            return false;
        } else {
            BaseResponse<?, ?> other = (BaseResponse)o;
            if (!other.canEqual(this)) {
                return false;
            } else {
                label107: {
                    Object this$success = this.getSuccess();
                    Object other$success = other.getSuccess();
                    if (this$success == null) {
                        if (other$success == null) {
                            break label107;
                        }
                    } else if (this$success.equals(other$success)) {
                        break label107;
                    }

                    return false;
                }

                Object this$code = this.getCode();
                Object other$code = other.getCode();
                if (this$code == null) {
                    if (other$code != null) {
                        return false;
                    }
                } else if (!this$code.equals(other$code)) {
                    return false;
                }

                Object this$message = this.getMessage();
                Object other$message = other.getMessage();
                if (this$message == null) {
                    if (other$message != null) {
                        return false;
                    }
                } else if (!this$message.equals(other$message)) {
                    return false;
                }

                label86: {
                    Object this$data = this.getData();
                    Object other$data = other.getData();
                    if (this$data == null) {
                        if (other$data == null) {
                            break label86;
                        }
                    } else if (this$data.equals(other$data)) {
                        break label86;
                    }

                    return false;
                }

                label79: {
                    Object this$extend = this.getExtend();
                    Object other$extend = other.getExtend();
                    if (this$extend == null) {
                        if (other$extend == null) {
                            break label79;
                        }
                    } else if (this$extend.equals(other$extend)) {
                        break label79;
                    }

                    return false;
                }

                label72: {
                    Object this$debugMessage = this.getDebugMessage();
                    Object other$debugMessage = other.getDebugMessage();
                    if (this$debugMessage == null) {
                        if (other$debugMessage == null) {
                            break label72;
                        }
                    } else if (this$debugMessage.equals(other$debugMessage)) {
                        break label72;
                    }

                    return false;
                }

                Object this$debugTrace = this.getDebugTrace();
                Object other$debugTrace = other.getDebugTrace();
                if (this$debugTrace == null) {
                    if (other$debugTrace != null) {
                        return false;
                    }
                } else if (!this$debugTrace.equals(other$debugTrace)) {
                    return false;
                }

                Object this$requestId = this.getRequestId();
                Object other$requestId = other.getRequestId();
                if (this$requestId == null) {
                    if (other$requestId != null) {
                        return false;
                    }
                } else if (!this$requestId.equals(other$requestId)) {
                    return false;
                }

                return true;
            }
        }
    }

    protected boolean canEqual(final Object other) {
        return other instanceof BaseResponse;
    }

    @Override
    public int hashCode() {
        boolean prime = true;
        int result = 1;
        Object $success = this.getSuccess();
        result = result * 59 + ($success == null ? 43 : $success.hashCode());
        Object $code = this.getCode();
        result = result * 59 + ($code == null ? 43 : $code.hashCode());
        Object $message = this.getMessage();
        result = result * 59 + ($message == null ? 43 : $message.hashCode());
        Object $data = this.getData();
        result = result * 59 + ($data == null ? 43 : $data.hashCode());
        Object $extend = this.getExtend();
        result = result * 59 + ($extend == null ? 43 : $extend.hashCode());
        Object $debugMessage = this.getDebugMessage();
        result = result * 59 + ($debugMessage == null ? 43 : $debugMessage.hashCode());
        Object $debugTrace = this.getDebugTrace();
        result = result * 59 + ($debugTrace == null ? 43 : $debugTrace.hashCode());
        Object $requestId = this.getRequestId();
        result = result * 59 + ($requestId == null ? 43 : $requestId.hashCode());
        return result;
    }

    @Override
    public String toString() {
        return "BaseResponse(success=" + this.getSuccess() + ", code=" + this.getCode() + ", message=" + this.getMessage() + ", data=" + this.getData() + ", extend=" + this.getExtend() + ", debugMessage=" + this.getDebugMessage() + ", debugTrace=" + this.getDebugTrace() + ", requestId=" + this.getRequestId() + ")";
    }
}
