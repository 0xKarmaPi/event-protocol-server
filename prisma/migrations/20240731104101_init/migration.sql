-- CreateTable
CREATE TABLE "Event" (
    "id" SERIAL NOT NULL,
    "title" TEXT NOT NULL,
    "endTime" TIMESTAMP(3) NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "Event_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "EventOption" (
    "id" SERIAL NOT NULL,
    "value" TEXT NOT NULL,
    "EventId" INTEGER NOT NULL,

    CONSTRAINT "EventOption_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "EventOption" ADD CONSTRAINT "EventOption_EventId_fkey" FOREIGN KEY ("EventId") REFERENCES "Event"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
