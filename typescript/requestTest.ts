import { request } from "undici";

async function makeRequest() {
  const url = "https://example.com/";

  const start = process.hrtime.bigint();

  try {
    const { statusCode, body } = await request(url);

    console.log(body);

    console.log("Status Code:", statusCode);

    const end = process.hrtime.bigint();
    console.log(Number(end - start) / 1e6);
    return Number(end - start) / 1e6; // Süreyi milisaniye cinsinden döndür
  } catch (error) {
    console.error("Error:", error);
    return 0; // Hata durumunda 0 döndür
  }
}

async function measureAverageRequestTime(iterations: number) {
  let totalDuration = 0;

  for (let i = 0; i < iterations; i++) {
    const duration = await makeRequest();
    totalDuration += duration;
  }

  const averageDuration = totalDuration / iterations;
  console.log(
    `Ortalama İstek Süresi (${iterations} iterasyon): ${averageDuration.toFixed(
      2
    )} ms`
  );
}

measureAverageRequestTime(100);
