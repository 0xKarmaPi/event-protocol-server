-- CreateTable
CREATE TABLE "PredictionEvent" (
    "id" SERIAL NOT NULL,
    "title" TEXT NOT NULL,
    "endTime" TIMESTAMP(3) NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "PredictionEvent_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "PredictionAnswer" (
    "id" SERIAL NOT NULL,
    "value" TEXT NOT NULL,
    "predictionEventId" INTEGER NOT NULL,

    CONSTRAINT "PredictionAnswer_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "PredictionAnswer" ADD CONSTRAINT "PredictionAnswer_predictionEventId_fkey" FOREIGN KEY ("predictionEventId") REFERENCES "PredictionEvent"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
