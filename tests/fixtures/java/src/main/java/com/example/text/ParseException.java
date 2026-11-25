package com.example.text;

/** Thrown when input cannot be parsed. */
public final class ParseException extends RuntimeException {
    public ParseException(String message) {
        super(message);
    }
}
