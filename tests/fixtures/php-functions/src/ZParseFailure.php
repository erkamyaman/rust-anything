<?php

namespace Acme\Numbers {
    if (!class_exists(ParseFailure::class, false)) {
        abstract class ParseFailure extends \RuntimeException {}
    }
}
