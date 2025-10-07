# action will be run in python3 container
FROM python:3@sha256:fe841081ec55481496a4ab25e538833741295d57d2abdec8d38d74d65fb4715b
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
