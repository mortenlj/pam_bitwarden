# action will be run in python3 container
FROM python:3@sha256:7960a76b45493e8a3b87a0365e257c547b66e8d0c8cd629c029aec9c9e31ed5e
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
