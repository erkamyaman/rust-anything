<?php

namespace Acme\Numbers;

require_once __DIR__ . '/ZParseFailure.php';
require_once __DIR__ . '/ParseException.php';

function clamp(int $value, int $lo = 0, int $hi = 10): int
{
    if ($lo > $hi) {
        throw new \InvalidArgumentException('lo is above hi');
    }
    return max($lo, min($hi, $value));
}

/**
 * @param list<float> $xs
 */
function mean(array $xs): float
{
    if ($xs === []) {
        throw new \DomainException('mean of nothing');
    }
    return array_sum($xs) / count($xs);
}

function ratio(float $a, float $b): float
{
    return $a / $b;
}

function digit(string $text): int
{
    if (strlen($text) === 1 && $text >= '0' && $text <= '9') {
        return (int) $text;
    }
    throw new ParseException('not a digit');
}
