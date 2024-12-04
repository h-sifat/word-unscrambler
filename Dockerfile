FROM ubuntu:22.04

WORKDIR /app

COPY ./dist /app
RUN chmod +x /app/app


# Create a non-root user and group
RUN addgroup --system appgroup && adduser --system --ingroup appgroup appuser

# Change ownership of the /app directory to the new user
RUN chown -R appuser:appgroup /app

# Switch to the non-root user
USER appuser

ENV PORT=8080
ENV MODE=production
EXPOSE 8080

# run the app
CMD ["./app"]
