<?php

function makeRequest() {
    $url = "https://example.com/";

    $start = microtime(true);

    $response = file_get_contents($url);

    $data = json_decode($response, true);

    $end = microtime(true);

    return ($end - $start) * 1000;
}

function measureAverageRequestTime($iterations) {
    $totalDuration = 0;

    for ($i = 0; $i < $iterations; $i++) {
        $duration = makeRequest();
        $totalDuration += $duration;
    }

    $averageDuration = $totalDuration / $iterations;
    echo "Ortalama İstek Süresi ({$iterations} iterasyon): " . number_format($averageDuration, 2) . " ms\n";
}

measureAverageRequestTime(100);
