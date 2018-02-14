require(ggplot2)
require(data.table)
dt = read.csv("~/Documents/Projects/sketchy/dump.csv")
dt = data.table(dt)
dt[, avg := value/mass]
dt[, index := .I]
dt[, quantile := cumsum(mass)/sum(mass)]
dt[, quantum := mass/sum(mass)]
dt[, avg_diff := (avg - c(NA, avg[1:length(avg)-1]))]
dt[, quantile_diff := (quantile - c(NA, quantile[1:length(quantile)-1]))]
ggplot(dt, aes(y=mass, x = avg)) + geom_point()
ggplot(dt, aes(y=avg, x = quantile)) + geom_point()
ggplot(dt, aes(y=quantum, x = avg)) + geom_point() + geom_smooth(method='loess')

ggplot(dt, aes(x = avg, y = ..density.., weight = quantum)) + geom_histogram(binwidth = 20000)
