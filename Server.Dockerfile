# Image
FROM golang:1.20.1

# Define the work directory
WORKDIR ./server

# Copy the project folders into the container's working directory
COPY ./server .

# Create/update the go.sum file
RUN go mod tidy

# Build it
RUN go build -o server .

# Give executable privileges
RUN chmod +x server

# Run the backend server
CMD [ "./server" ]